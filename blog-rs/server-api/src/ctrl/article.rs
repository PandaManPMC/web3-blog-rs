use axum::{Json, Router};
use axum::extract::Query;
use axum::routing::{get};
use i_dao::sql;
use log::debug;
use base::model::blog_classes::BlogClassesModel;
use crate::{bean, service, utils};
use std::collections::HashMap;
use std::string;
use base::model::blog_label::BlogLabelModel;
use base::model::blog_view::BlogViewModel;
use crate::ctrl::PREIFIX;

pub fn init_router(mut router: Router) -> Router {
    router = router.route(&format!("{}{}", PREIFIX, "/article/list"), get(get_article_list));
    router = router.route(&format!("{}{}", PREIFIX, "/article/classes"), get(get_classes_list));
    router = router.route(&format!("{}{}", PREIFIX, "/article/comments"), get(get_article_comments));
    router = router.route(&format!("{}{}", PREIFIX, "/article/labels"), get(get_label_list));
    return router;
}

/// article_list 获取博客列表
async fn get_article_list(
    query: Query<bean::article::GetArticleList>,
) -> Json<common::net::rsp::Rsp<Vec<bean::article::BlogArticleOut>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    if 0 != query.id_blog_classes {
        params.insert(String::from("id_blog_classes"), sql::Params::UInteger64(query.id_blog_classes));
    } else {
        if 0 != query.id_blog_label {
            params.insert(String::from("id_blog_label"), sql::Params::UInteger64(query.id_blog_label));
        }
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

    // 查询作者
    let pn = service::blog::find_author_by_id(1).await;
    if pn.is_err() {
        tracing::warn!("{:?}", pn);
        return Json(common::net::rsp::Rsp::<Vec<bean::article::BlogArticleOut>>::err_de())
    }
    let pem_name = pn.unwrap();

    for article in lst {
        // 查询关联标签
        let mut params1:HashMap<String, sql::Params> = HashMap::new();
        params1.insert(String::from("id_blog_article"), sql::Params::UInteger64(article.id_blog_classes));
        params1.insert(String::from("state"), sql::Params::UInteger8(1));
        let res = base::service::blog_article_label_sve::query_list(&params, &utils::limit_max()).await;
        if res.is_err() {
            tracing::warn!("{:?}", res);
            return Json(common::net::rsp::Rsp::<Vec<bean::article::BlogArticleOut>>::err_de())
        }

        let lst1 = res.unwrap();

        let mut labels = vec![];
        for articleLabel in lst1 {
            let label = service::blog::find_label_by_id(articleLabel.id_blog_label).await;
            if !(label.is_err()) {
                let label_name = label.unwrap();
                labels.push(label_name);
                continue;
            }
            tracing::warn!("{:?}", label);
            return Json(common::net::rsp::Rsp::<Vec<bean::article::BlogArticleOut>>::err_de());
        }

        let target = bean::article::BlogArticleOut{
            id: article.id,
            created_at: article.created_at,
            updated_at: article.updated_at,
            id_blog_classes: article.id_blog_classes,
            title_article: article.title_article,
            state_article: article.state_article,
            state_publish: article.state_publish,
            state_private: article.state_private,
            content: article.content,
            like_count: article.like_count,
            watch_count: article.watch_count,
            view_count: article.view_count,
            time_publish: article.time_publish,
            sequence: article.sequence,
            pem_name: pem_name.clone(),
            labels,
        };
        list.push(target);
    }

    let rsp = common::net::rsp::Rsp::ok(list);
    Json(rsp)
}

/// 获取分类
async fn get_classes_list() -> Json<common::net::rsp::Rsp<Vec<BlogClassesModel>>> {
    // debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    // thing状态:1@可见;2@不可见
    params.insert(String::from("state"), sql::Params::UInteger8(1));

    let [page_index, page_size] = utils::limit_max();
    let desc = sql::Condition::OrderByField("sequence".to_string());
    let bc = [page_index, page_size, desc];

    let result = base::service::blog_classes_sve::query_list(&params, &bc).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogClassesModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

/// 获取标签
async fn get_label_list() -> Json<common::net::rsp::Rsp<Vec<BlogLabelModel>>> {
    let mut params:HashMap<String, sql::Params> = HashMap::new();
    // thing状态:1@可见;2@不可见
    params.insert(String::from("state"), sql::Params::UInteger8(1));

    let [page_index, page_size] = utils::limit_max();
    let desc = sql::Condition::OrderByField("sequence".to_string());
    let bc = [page_index, page_size, desc];

    let result = base::service::blog_label_sve::query_list(&params, &bc).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogLabelModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
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
