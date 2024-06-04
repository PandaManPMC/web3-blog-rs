use i_dao::i_mysql;
use iconf::configs;
use log::{warn, info, error};
use std::env;
use plier;
use r2d2_mysql::mysql::OptsBuilder;
use std::time::Duration;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    middleware
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use plier::rds;

mod ctrl;
mod service;
mod bean;

#[tokio::main]
async fn main() {
    let p = plier::files::get_current_dir_str();
    println!("{:?}", p);

    let file_path = String::from("server-manage/src/config.toml");

    unsafe {
        let res = configs::init(file_path.clone());
        log4rs::init_file("server-manage/src/log4rs.yaml", Default::default()).unwrap();
        info!("log4rs::init_file {:?}", file_path.clone());

        info!("env = {}", configs::get_str("basics", "env"));
        info!("port = {}", configs::get_int("basics", "port"));

        init_mysql();
        init_rds().await;

        let mut app = Router::new();
        app = init_router(app);

        info!("server = {:?}", format!("0.0.0.0:{}", configs::get_int("basics", "port")));

        init_author();

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", configs::get_int("basics", "port"))).await.unwrap();
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    }

}

async fn init_rds(){
    let url = "redis://rust1:rust1_abc1234@34.118.182.28:26379/0";
    let size: usize = 2;

    let res_rds = rds::init_rds(url, size).await;
    info!("res_rds={:?}", res_rds);

    let rds = res_rds.unwrap();

    common::cache::member_rds::initialize_global_object(rds).await;

    let res = common::cache::member_rds::get_user_by_token("abc".to_string()).await;
    info!("init_rds get_user_by_token={:?}", res);
}

/// init_router 初始化路由
fn init_router(mut router: Router) -> Router {
    router = router.route("/", get(root));
    router = ctrl::admin::init_router(router);
    router = ctrl::article::init_router(router);
    router = router.layer(middleware::from_fn(common::net::interceptor::error_handling));
    router  = router.layer(middleware::from_fn(ctrl::interceptor::app));
    // router.recover(ctrl::handle_error::custom_error_handler);
    return router;
}

async fn root() -> &'static str {
    "web3 blog by rust"
}

/// init_mysql 初始化 mysql
unsafe fn init_mysql() {
    base::service::set_date_source_key(String::from("mysql_db1"));
    service::set_date_source_key(String::from("mysql_db1"));

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(configs::get_str("mysql_db1", "host")))
        .user(Some(configs::get_str("mysql_db1", "dbname")))
        .pass(Some(configs::get_str("mysql_db1", "password")))
        .db_name(Some(configs::get_str("mysql_db1", "dbname")))
        .tcp_port(configs::get_int("mysql_db1", "port") as u16)
        .tcp_connect_timeout(Some(Duration::from_secs(configs::get_int("mysql_db1", "connect_timeout").try_into().unwrap())));

    i_mysql::init(base::service::get_data_source_key(), opts, configs::get_int("mysql_db1", "max_size") as u32, configs::get_int("mysql_db1", "max_idle") as u32);

    let conn = i_mysql::get_conn(&base::service::get_data_source_key());
    if conn.is_err() {
        warn!("init_mysql {:?}", conn);
        panic!("init_mysql");
    }
}

unsafe fn init_author() {
    let username = configs::get_str("author", "username");
    let user_pwd = configs::get_str("author", "userpwd");
    let pen_name = configs::get_str("author", "penname");


    let d_au = base::service::blog_author_sve::find_by_user_name(username.clone());
    if d_au.is_err() {
        error!("find author error by username = {:?}", username);
        return;
    }

    let u = d_au.unwrap();

    if let Some(a) = u {
        info!("author = {:?} 已存在", a.pen_name);
        return;
    }

    // 不存在作者，创建
    let mut author = base::model::blog_author::BlogAuthorModel::new(
        pen_name, username,
        plier::md::sha256(user_pwd), "".to_string());

    let res = base::service::blog_author_sve::add(&mut author);
    if res.is_err() {
        warn!("创建默认作者失败！");
        return;
    }

    info!("作者={:?}-{:?}，创建完成。", author.pen_name, author.id);
}
