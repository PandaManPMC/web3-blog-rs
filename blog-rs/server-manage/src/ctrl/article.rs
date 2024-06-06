use std::any::Any;
use std::collections::HashMap;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    extract::Query,
};
use log::debug;
use crate::bean;
use i_dao::sql;
use base::model::blog_article::BlogArticleModel;
use base::model::blog_classes::BlogClassesModel;
use base::model::blog_label::BlogLabelModel;

pub fn init_router(mut router: Router) -> Router {
    // router = router.route("/article/publish", post(publish));
    router = router.route("/article/getArticleLst", get(get_article_lst));
    router = router.route("/article/getClassesLst", get(get_classes_lst));
    router = router.route("/article/getLabelLst", get(get_label_lst));

    return router;
}

// async fn publish(
//     Json(payload): Json<bean::admin::LoginIn>,
// ) -> (StatusCode, Json<bean::admin::LoginOut>) {
//
//     debug!("{:?}", payload);
//
//
//
//     (StatusCode::CREATED, Json(out))
// }

async fn get_article_lst(
    query: Query<bean::article::GetArticleLstIn>,
) -> Json<common::net::rsp::Rsp<Vec<BlogArticleModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
    if 0 != query.state_article {
        params.insert(String::from("state_article"), Box::new(query.state_article));
    }

    if 0 != query.state_private {
        params.insert(String::from("state_private"), Box::new(query.state_private));
    }

    if 0 != query.state_publish {
        params.insert(String::from("state_publish"), Box::new(query.state_publish));
    }

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let bc = [page_index, page_size, ];

    let result = base::service::blog_article_sve::query_list(&params, &bc);

    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogArticleModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

async fn get_classes_lst(
    query: Query<bean::article::GetClassesLstIn>,
) -> Json<common::net::rsp::Rsp<Vec<BlogClassesModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
    if 0 != query.state {
        params.insert(String::from("state"), Box::new(query.state));
    }

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let bc = [page_index, page_size, ];

    let result = base::service::blog_classes_sve::query_list(&params, &bc);

    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogClassesModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

async fn get_label_lst(
    query: Query<bean::article::GetLabelLstIn>,
) -> Json<common::net::rsp::Rsp<Vec<BlogLabelModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
    if 0 != query.state {
        params.insert(String::from("state"), Box::new(query.state));
    }

    let page_index = sql::Condition::PageIndex(query.page_index);
    let page_size = sql::Condition::PageSize(query.page_size);
    let bc = [page_index, page_size, ];

    let result = base::service::blog_label_sve::query_list(&params, &bc);

    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogLabelModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}