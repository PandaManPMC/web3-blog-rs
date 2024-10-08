use axum::{Json, Router};
use axum::extract::Query;
use axum::routing::{get, post};
use i_dao::sql;
use log::debug;
use base::model::blog_classes::BlogClassesModel;
use crate::{bean, service, utils};
use std::collections::HashMap;
use std::{result, string};
use base::model::blog_label::BlogLabelModel;
use base::model::blog_view::BlogViewModel;
use crate::bean::article::{BlogClassesOut, BlogLabelOut};
use crate::ctrl::PREIFIX;
use tokio::sync::Mutex;
use std::sync::Arc;
use axum::http::HeaderMap;

lazy_static::lazy_static! {
    static ref LOCK: Arc<Mutex<bool>> = Arc::new(Mutex::new({
        true
    }));
}

pub fn init_router(mut router: Router) -> Router {
    router = router.route(&format!("{}{}", PREIFIX, "/article/read"), get(read));
    router = router.route(&format!("{}{}", PREIFIX, "/article/list"), get(get_article_list));
    router = router.route(&format!("{}{}", PREIFIX, "/article/classes"), get(get_classes_list));
    router = router.route(&format!("{}{}", PREIFIX, "/article/comments"), get(get_article_comments));
    router = router.route(&format!("{}{}", PREIFIX, "/article/labels"), get(get_label_list));
    router = router.route(&format!("{}{}", PREIFIX, "/article/views"), get(get_view_list));
    router = router.route(&format!("{}{}", PREIFIX, "/article/createView"), post(create_view));
    router = router.route(&format!("{}{}", PREIFIX, "/article/getViewTicket"), get(get_view_ticket));

    return router;
}

/// read 读取博客
async fn read(
    query: Query<bean::article::ReadIn>,
) -> Json<common::net::rsp::Rsp<bean::article::ReadOut>> {
    let result = base::service::blog_article_sve::find_by_id(query.id).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<bean::article::ReadOut>::err_de())
    }

    let res = result.unwrap();
    if res.is_none() {
        return Json(common::net::rsp::Rsp::fail("文章不存在".to_string()))
    }
    let article = res.unwrap();
    if 1 != article.state_article || 2 != article.state_publish || 2 != article.state_private {
        return Json(common::net::rsp::Rsp::fail("文章不存在".to_string()))
    }

    {
        let r = service::blog::update_watch_count(article.id, article.watch_count+1).await;
        if r.is_err() {
            tracing::warn!("update_watch_count {:?}", r);
        }
    }

    // 查询关联标签
    let mut params1:HashMap<String, sql::Params> = HashMap::new();
    params1.insert(String::from("id_blog_article"), sql::Params::UInteger64(article.id));
    params1.insert(String::from("state"), sql::Params::UInteger8(1));
    let res = base::service::blog_article_label_sve::query_list(&params1, &utils::limit_max()).await;
    if res.is_err() {
        tracing::warn!("{:?}", res);
        return Json(common::net::rsp::Rsp::<bean::article::ReadOut>::err_de())
    }

    let lst1 = res.unwrap();
    let mut labels = vec![];
    for articleLabel in lst1 {
        let label = service::blog::find_label_by_id(articleLabel.id_blog_label).await;
        if !label.is_err() {
            let label_name = label.unwrap();
            labels.push(label_name);
        }
    }

    let author = service::BLOG_AUTHOR.get().expect("BLOG_AUTHOR should be initialized").read().await;

    let mut classes_name = "".to_string();
    {

        let r = service::blog::find_classes_name_by_id(article.id_blog_classes).await;
        if r.is_ok() {
            classes_name = r.unwrap();
        }
    }

    let res = bean::article::ReadOut{
        id: article.id,
        id_blog_classes: article.id_blog_classes,
        title_article: article.title_article,
        content: article.content,
        like_count: article.like_count,
        watch_count: article.watch_count,
        view_count: article.view_count,
        time_publish: article.time_publish,
        sequence: article.sequence,
        pem_name: author.pen_name.clone(),
        profile: author.profile.clone(),
        introduce: author.introduce.clone(),
        mk_footer: author.mk_footer.clone(),
        labels,
        classes_name
    };

    return Json(common::net::rsp::Rsp::ok(res));
}

/// article_list 获取博客列表
async fn get_article_list(
    query: Query<bean::article::GetArticleList>,
) -> Json<common::net::rsp::Rsp<Vec<bean::article::BlogArticleOut>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    if 0 != query.id_blog_classes {
        params.insert(String::from("id_blog_classes"), sql::Params::UInteger64(query.id_blog_classes));
    }

    if 0 != query.id_blog_label {
        params.insert(String::from("id_blog_label"), sql::Params::UInteger64(query.id_blog_label));
    }

    // thing状态:1@正常;2@已删除
    params.insert(String::from("state_article"), sql::Params::UInteger8(1));
    // thing文章可见性:1@私有;2@公开
    params.insert(String::from("state_private"), sql::Params::UInteger8(2));
    // thing发布状态:1@未发布;2@已发布
    params.insert(String::from("state_publish"), sql::Params::UInteger8(2));

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let desc = sql::Condition::OrderByField("sequence".to_string());

    let bc = [page_index, page_size, desc ];

    let result = service::blog::query_list(&params, &bc).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<bean::article::BlogArticleOut>>::err_de())
    }

    let lst = result.clone().unwrap();

    let mut list: Vec<bean::article::BlogArticleOut> = vec![];

    let (pem_name,profile) = service::get_author_info().await;

    for article in lst {
        // 查询关联标签
        let mut params1:HashMap<String, sql::Params> = HashMap::new();
        params1.insert(String::from("id_blog_article"), sql::Params::UInteger64(article.id));
        params1.insert(String::from("state"), sql::Params::UInteger8(1));
        let res = base::service::blog_article_label_sve::query_list(&params1, &utils::limit_max()).await;
        if res.is_err() {
            tracing::warn!("{:?}", res);
            return Json(common::net::rsp::Rsp::<Vec<bean::article::BlogArticleOut>>::err_de())
        }

        let lst1 = res.unwrap();

        let mut labels = vec![];
        for articleLabel in lst1 {
            let label = service::blog::find_label_by_id(articleLabel.id_blog_label).await;
            if !label.is_err() {
                let label_name = label.unwrap();
                labels.push(label_name);
            }
        }

        let target = bean::article::BlogArticleOut{
            id: article.id,
            id_blog_classes: article.id_blog_classes,
            title_article: article.title_article,
            content: article.content,
            like_count: article.like_count,
            watch_count: article.watch_count,
            view_count: article.view_count,
            time_publish: article.time_publish,
            sequence: article.sequence,
            pem_name: pem_name.clone(),
            profile: profile.clone(),
            labels,
        };
        list.push(target);
    }

    let rsp = common::net::rsp::Rsp::ok(list);
    Json(rsp)
}

/// 获取分类
async fn get_classes_list() -> Json<common::net::rsp::Rsp<Vec<BlogClassesOut>>> {
    let mut lst: Vec<BlogClassesOut> = Vec::new();
    let cache = service::CLASSES_LIST.read().await;
    for (key, value) in cache.iter() {
        lst.push(crate::bean::article::BlogClassesOut{
            id: value.id,
            classes_name: value.classes_name.clone(),
            sequence: value.sequence,
        })
    }
    lst.sort_by_key(|p| std::cmp::Reverse(p.sequence));
    Json(common::net::rsp::Rsp::ok(lst))
}

/// 获取标签
async fn get_label_list() -> Json<common::net::rsp::Rsp<Vec<BlogLabelOut>>> {
    let mut labels: Vec<BlogLabelOut> = Vec::new();
    let cache = service::LABEL_LIST.read().await;
    for (key, value) in cache.iter() {
        labels.push(crate::bean::article::BlogLabelOut{
            id: value.id,
            label_name: value.label_name.clone(),
            sequence: value.sequence,
        })
    }
    labels.sort_by_key(|p| std::cmp::Reverse(p.sequence));
    Json(common::net::rsp::Rsp::ok(labels))
}

/// 获取文章评论
async fn get_article_comments(
    query: Query<bean::article::GetArticleComment>,
) -> Json<common::net::rsp::Rsp<Vec<BlogViewModel>>> {
    if 0 == query.id_blog {
        return Json(common::net::rsp::Rsp::fail_params_tip("缺少必传参数【id_blog】".to_string()))
    }
    let mut params:HashMap<String, sql::Params> = HashMap::new();
    // thing状态:1@可见;2@不可见
    params.insert(String::from("visible"), sql::Params::UInteger8(1));
    params.insert(String::from("id_blog_article"), sql::Params::UInteger64(query.id_blog));

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);

    let bc = [page_index, page_size];

    let result = base::service::blog_view_sve::query_list(&params, &bc).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogViewModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

/// article_list 获取博客列表
async fn get_view_list(
    query: Query<bean::article::GetViewListIn>,
) -> Json<common::net::rsp::Rsp<Vec<bean::article::BlogViewOut>>> {

    debug!("{:?}", query);
    let mut params:HashMap<String, sql::Params> = HashMap::new();
    params.insert(String::from("id_blog_article"), sql::Params::UInteger64(query.id_blog));
    params.insert(String::from("visible"), sql::Params::UInteger64(1));

    let result = base::service::blog_view_sve::query_list(&params, &utils::limit_max()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<bean::article::BlogViewOut>>::err_de())
    }

    let lst = result.clone().unwrap();

    let mut list: Vec<bean::article::BlogViewOut> = vec![];

    let (pem_name,profile) = service::get_author_info().await;

    for view in lst {
        let target = bean::article::BlogViewOut{
            created_at: view.created_at,
            view_content: view.view_content,
            coin_symbol: view.coin_symbol,
            tip_amount: view.tip_amount,
            address: view.address,
            tip_amount_usd: view.tip_amount_usd,
            ticket: view.ticket,
        };
        list.push(target);
    }

    let rsp = common::net::rsp::Rsp::ok(list);
    Json(rsp)
}

/// get_view_ticket 获取评论票据
async fn get_view_ticket(
    query: Query<bean::article::GetViewTicketIn>,
) -> Json<common::net::rsp::Rsp<bean::article::GetViewTicketOut>> {
    let uid = plier::uid::uid_v4();
    let result = common::cache::common_rds::set_string_expire(uid.clone(), query.address.clone(), 60000).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<bean::article::GetViewTicketOut>::err_de())
    }

    return Json(common::net::rsp::Rsp::<bean::article::GetViewTicketOut>::ok(bean::article::GetViewTicketOut{ ticket: uid }));
}

/// create_view 创建评论
async fn create_view (
    headers: HeaderMap,
    Json(payload): Json<bean::article::CreateViewIn>,
) -> Json<common::net::rsp::Rsp<bean::article::BlogViewOut>> {

    let real_ip = common::net::get_client_real_ip(&headers);
    tracing::info!("create_view 访问者 ip={:?}", real_ip);
    debug!("{:?}", payload);

    let _ = LOCK.lock().await;

    let get_addr = common::cache::common_rds::get_string(payload.ticket.clone()).await;
    if get_addr.is_err() {
        tracing::warn!("{:?}", get_addr);
        return Json(common::net::rsp::Rsp::<bean::article::BlogViewOut>::err_de())
    }

    let address = get_addr.unwrap();
    if "" == address {
        return Json(common::net::rsp::Rsp::<bean::article::BlogViewOut>::fail("票据不存在".to_string()));
    }

    // 调用合约核实
    let get_addr = common::tool::contract::get_address(payload.ticket.clone()).await;
    if get_addr.is_err() {
        tracing::warn!("{:?}", get_addr);
        return Json(common::net::rsp::Rsp::<bean::article::BlogViewOut>::err_de())
    }
    let get_add = get_addr.unwrap();
    if "" == get_add {
        return Json(common::net::rsp::Rsp::fail("票据尚未支付，无法提交评论".to_string()))
    }
    tracing::info!("合约核实 {} - {} 已支付", payload.ticket, get_add);

    let result = base::service::blog_article_sve::find_by_id(payload.id_blog).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<bean::article::BlogViewOut>::err_de())
    }

    let res = result.unwrap();
    if res.is_none() {
        return Json(common::net::rsp::Rsp::fail("文章不存在".to_string()))
    }
    let article = res.unwrap();
    if 1 != article.state_article || 2 != article.state_publish || 2 != article.state_private {
        return Json(common::net::rsp::Rsp::fail("文章不存在".to_string()))
    }

    let amount = "0.01";
    let mut amount_usd = "0.01".to_string();
    let usd = service::currency::get_price_by_matic(amount.to_string()).await;
    if usd.is_err() {
        tracing::warn!("{:?}", usd);
    }else{
        amount_usd = usd.unwrap().to_string();
    }

    let mut view = base::model::blog_view::BlogViewModel::new(
        payload.id_blog,
        payload.view_content,
        payload.coin_symbol,
        amount.to_string(),
    1,
        address,
        amount_usd,
        real_ip,
        payload.ticket.clone());

    let result = service::blog::add_view(payload.id_blog, article.view_count+1 , &mut view).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<bean::article::BlogViewOut>::err_de())
    }

    {
        let r = common::cache::common_rds::del_string(payload.ticket).await;
        if r.is_err() {
            tracing::warn!("{:?}", r);
        }
    }

    return Json(common::net::rsp::Rsp::<bean::article::BlogViewOut>::ok(bean::article::BlogViewOut{
        created_at: view.created_at,
        view_content: view.view_content,
        coin_symbol: view.coin_symbol,
        tip_amount: view.tip_amount,
        address: view.address,
        tip_amount_usd: view.tip_amount_usd,
        ticket: view.ticket,
    }));
}