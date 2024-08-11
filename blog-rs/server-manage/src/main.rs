// use i_dao::i_mysql;
use i_dao::tok::i_mysql;
use iconf::configs;
use log::{warn, info, error, debug};
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
use axum::extract::DefaultBodyLimit;
use tracing::field::debug;

mod ctrl;
mod service;
mod bean;
mod tool;

#[tokio::main]
async fn main() {
    let p = plier::files::get_current_dir_str();
    println!("{:?}", p);

    let mut conf_file_path = String::from("server-manage/src/config.toml");
    let mut log_conf_path = String::from("server-manage/src/log4rs.yaml");

    if cfg!(target_os = "linux") {
        conf_file_path = String::from("./config.toml");
        log_conf_path = String::from("./log4rs.yaml");
    }

    unsafe {
        let res = configs::init(conf_file_path.clone());
        log4rs::init_file(log_conf_path.clone(), Default::default()).unwrap();
        info!("log4rs::init_file {:?}", log_conf_path.clone());

        info!("env = {}", configs::get_str("basics", "env"));
        info!("port = {}", configs::get_int("basics", "port"));

        init_mysql().await;
        init_rds().await;

        let mut app = Router::new();
        app = init_router(app);

        info!("server = {:?}", format!("0.0.0.0:{}", configs::get_int("basics", "port")));

        init_author().await;

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", configs::get_int("basics", "port"))).await.unwrap();
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    }

}

async fn init_rds(){
    unsafe {
        // let url = "redis://username:pwd@host:port/db";

        let url = &format!("redis://{}:{}@{}:{}/{}",
                           configs::get_str("redis", "username"),
                           configs::get_str("redis", "pwd"),
                           configs::get_str("redis", "host"),
                           configs::get_str("redis", "port"),
                           configs::get_str("redis", "db"));

        let size: usize = 2;

        let res_rds = rds::init_rds(url, size).await;
        info!("res_rds={:?}", res_rds);

        let rds = res_rds.unwrap();

        common::cache::member_rds::initialize_global_object(rds).await;

        let res = common::cache::member_rds::get_user_by_token("abc".to_string()).await;
        info!("init_rds get_user_by_token={:?}", res);
    }
}

/// init_router 初始化路由
fn init_router(mut router: Router) -> Router {
    router = router.route("/", get(root));
    router = ctrl::test::init_router(router);
    router = ctrl::admin::init_router(router);
    router = ctrl::article::init_router(router);
    router = ctrl::common::init_router(router);
    router = router.layer(middleware::from_fn(common::net::interceptor::error_handling));
    router = router.layer(middleware::from_fn(ctrl::interceptor::app));
    router = router.layer(DefaultBodyLimit::max(100 * 1024 * 1024));
    return router;
}

async fn root() -> &'static str {
    "web3 blog by rust"
}

/// init_mysql 初始化 mysql
async unsafe fn init_mysql() {
    base::service::set_date_source_key(String::from("mysql_db1")).await;
    service::set_date_source_key(String::from("mysql_db1")).await;

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(configs::get_str("mysql_db1", "host")))
        .user(Some(configs::get_str("mysql_db1", "username")))
        .pass(Some(configs::get_str("mysql_db1", "password")))
        .db_name(Some(configs::get_str("mysql_db1", "dbname")))
        .tcp_port(configs::get_int("mysql_db1", "port") as u16)
        .tcp_connect_timeout(Some(Duration::from_secs(configs::get_int("mysql_db1", "connect_timeout").try_into().unwrap())));

    debug!("{:?}", opts);

    i_mysql::init(base::service::get_data_source_key().await, opts, configs::get_int("mysql_db1", "max_size") as u32, configs::get_int("mysql_db1", "max_idle") as u32).await;

    let conn = i_mysql::get_conn(&base::service::get_data_source_key().await).await;
    // let conn = i_mysql::get_conn(&service::get_data_source_key().await).await;

    if conn.is_err() {
        warn!("init_mysql {:?}", conn);
        panic!("init_mysql");
    }
}

async unsafe fn init_author() {
    let username = configs::get_str("author", "username");
    let user_pwd = configs::get_str("author", "userpwd");
    let pen_name = configs::get_str("author", "penname");
    let profile = configs::get_str("author", "profile");
    let introduce = configs::get_str("author", "introduce");
    let contact_mail = configs::get_str("author", "contactMail");

    let d_au = base::service::blog_author_sve::find_by_user_name(username.clone()).await;
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
        pen_name,
        username,
        plier::md::sha256(user_pwd),
        "".to_string(),
        profile,
        introduce,
        "".to_string(),
        contact_mail);

    let res = base::service::blog_author_sve::add(&mut author).await;
    if res.is_err() {
        warn!("创建默认作者失败！");
        return;
    }

    info!("作者={:?}-{:?}，创建完成。", author.pen_name, author.id);
}
