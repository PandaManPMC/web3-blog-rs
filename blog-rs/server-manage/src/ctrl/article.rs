use std::any::Any;
use std::collections::HashMap;
use axum::{
    routing::{get, post},
    Json, Router,
    extract::Query,
    http::{StatusCode, header::HeaderMap, header::HeaderValue},
};
use log::{debug, info};
use crate::{bean, tool};
use i_dao::sql;
use base::model::blog_article::BlogArticleModel;
use base::model::blog_classes::BlogClassesModel;
use base::model::blog_label::BlogLabelModel;
use tokio::sync::RwLock;
use std::sync::Arc;

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/article/publish", post(publish));
    router = router.route("/article/createClasses", post(create_classes));
    router = router.route("/article/createLabel", post(create_label));

    router = router.route("/article/getArticleLst", get(get_article_lst));
    router = router.route("/article/getClassesLst", get(get_classes_lst));
    router = router.route("/article/getLabelLst", get(get_label_lst));
    return router;
}

/// publish 发布新文章
async fn publish(
    headers: HeaderMap,
    Json(payload): Json<bean::article::PublishIn>,
) -> Json<common::net::rsp::Rsp<u64>> {
    debug!("{:?}", payload);

    let now = plier::time::unix_second();
    let mut article = BlogArticleModel::new(tool::req::get_user_id(&headers),
                          payload.id_blog_classes,
                          payload.title_article, 1,
                          payload.state_publish, payload.state_private,
                          payload.content, 0,0,0, now, payload.sequence);

    let res = base::service::blog_article_sve::add(&mut article).await;
    if res.is_err() {
        tracing::warn!("{:?}", res);
        return Json(common::net::rsp::Rsp::<u64>::err_de())
    }

    return Json(common::net::rsp::Rsp::<u64>::ok(article.id));
}

/// get_article_lst 文章列表
async fn get_article_lst(
    query: Query<bean::article::GetArticleLstIn>,
) -> Json<common::net::rsp::Rsp<Vec<BlogArticleModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    if 0 != query.id_blog_classes {
        params.insert(String::from("id_blog_classes"), sql::Params::UInteger64(query.id_blog_classes));
    }

    if 0 != query.state_article {
        params.insert(String::from("state_article"), sql::Params::UInteger8(query.state_article));
    }

    if 0 != query.state_private {
        params.insert(String::from("state_private"), sql::Params::UInteger8(query.state_private));
    }

    if 0 != query.state_publish {
        params.insert(String::from("state_publish"), sql::Params::UInteger8(query.state_publish));
    }

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let desc = sql::Condition::OrderByField("sequence".to_string());

    let bc = [page_index, page_size, desc ];

    let result = base::service::blog_article_sve::query_list(&params, &bc).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogArticleModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

/// create_classes 创建文章类型
async fn create_classes (
    Json(payload): Json<bean::article::CreateClassesIN>,
) -> Json<common::net::rsp::Rsp<u64>> {
    debug!("{:?}", payload);

    let r = base::service::blog_classes_sve::find_by_classes_name(payload.classes_name.clone()).await;
    if r.is_err(){
        tracing::warn!("{:?}", r);
        return Json(common::net::rsp::Rsp::<u64>::err_de())
    }

    if r.unwrap().is_some() {
        return Json(common::net::rsp::Rsp::<u64>::fail("该类型已存在".to_string()))
    }

    let mut cla = BlogClassesModel::new(payload.classes_name, 1, payload.sequence);
    let res = base::service::blog_classes_sve::add(&mut cla).await;
    if res.is_err() {
        tracing::warn!("{:?}", res);
        return Json(common::net::rsp::Rsp::<u64>::err_de())
    }
    return Json(common::net::rsp::Rsp::<u64>::ok(cla.id));
}

/// get_classes_lst 读取文章类型列表
async fn get_classes_lst(
    query: Query<bean::article::GetClassesLstIn>,
) -> Json<common::net::rsp::Rsp<Vec<BlogClassesModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    if 0 != query.state {
        params.insert(String::from("state"), sql::Params::UInteger8(query.state));
    }

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let desc = sql::Condition::OrderByField("sequence".to_string());

    let bc = [page_index, page_size, desc ];

    let result = base::service::blog_classes_sve::query_list(&params, &bc).await;

    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogClassesModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

/// create_label 创建标签
async fn create_label (
    Json(payload): Json<bean::article::CreateLabelIn>,
) -> Json<common::net::rsp::Rsp<u64>> {
    debug!("{:?}", payload);

    let r = base::service::blog_label_sve::find_by_label_name(payload.label_name.clone()).await;
    if r.is_err(){
        tracing::warn!("{:?}", r);
        return Json(common::net::rsp::Rsp::<u64>::err_de())
    }

    if r.unwrap().is_some() {
        return Json(common::net::rsp::Rsp::<u64>::fail("该标签已存在".to_string()))
    }

    let mut cla = BlogLabelModel::new(payload.label_name, 1, payload.sequence);
    let res = base::service::blog_label_sve::add(&mut cla).await;
    if res.is_err() {
        tracing::warn!("{:?}", res);
        return Json(common::net::rsp::Rsp::<u64>::err_de())
    }
    return Json(common::net::rsp::Rsp::<u64>::ok(cla.id));
}

/// get_label_lst 获取标签列表
async fn get_label_lst(
    query: Query<bean::article::GetLabelLstIn>,
) -> Json<common::net::rsp::Rsp<Vec<BlogLabelModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    if 0 != query.state {
        params.insert(String::from("state"), sql::Params::UInteger8(query.state));
    }

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let desc = sql::Condition::OrderByField("sequence".to_string());

    let bc = [page_index, page_size, desc ];

    let result = base::service::blog_label_sve::query_list(&params, &bc).await;

    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogLabelModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}