use axum::http::HeaderMap;
use axum::{Json, Router};
use axum::extract::Query;
use axum::routing::{get};
use i_dao::sql;
use log::debug;
use base::model::blog_article::BlogArticleModel;
use base::model::blog_classes::BlogClassesModel;
use crate::{bean, utils};
use std::collections::HashMap;
use base::model::blog_label::BlogLabelModel;

pub fn init_router(mut router: Router) -> Router {
    router = router.route("/article/list", get(get_article_list));
    router = router.route("/article/classes", get(get_classes_list));
    return router;
}

/// article_list 获取博客列表
async fn get_article_list(
    query: Query<bean::article::GetArticleList>,
) -> Json<common::net::rsp::Rsp<Vec<BlogArticleModel>>> {

    debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    if 0 != query.id_blog_classes {
        params.insert(String::from("id_blog_classes"), sql::Params::UInteger64(query.id_blog_classes));
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

    let result = base::service::blog_article_sve::query_list(&params, &bc).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return Json(common::net::rsp::Rsp::<Vec<BlogArticleModel>>::err_de())
    }

    let lst = result.unwrap();
    let rsp = common::net::rsp::Rsp::ok(lst);
    Json(rsp)
}

/// 获取分类
async fn get_classes_list() -> Json<common::net::rsp::Rsp<Vec<BlogClassesModel>>> {
    // debug!("{:?}", query);

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    // thing状态:1@可见;2@不可见
    params.insert(String::from("state"), sql::Params::UInteger8(1));

    let [page_index, page_size] = utils::limit_min();
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

    let [page_index, page_size] = utils::limit_min();
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

