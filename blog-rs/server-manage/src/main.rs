use i_dao::i_mysql;
use iconf::configs;
use log::{warn,info};
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

mod ctrl;
mod service;
mod bean;

#[tokio::main]
async fn main() {
    let p = plier::files::get_current_dir_str();
    println!("{:?}", p);

    let file_path = String::from("server-manage/src/config.toml");

    unsafe {
        let res = configs::init(file_path);
        log4rs::init_file("server-manage/src/log4rs.yaml", Default::default()).unwrap();
        info!("log4rs::init_file");

        init_mysql();

        info!("env = {}", configs::get_str("basics", "env"));
        info!("port = {}", configs::get_int("basics", "port"));

        let mut app = Router::new();
        app = init_router(app);

        info!("server = {:?}", format!("0.0.0.0:{}", configs::get_int("basics", "port")));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", configs::get_int("basics", "port"))).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

}

/// init_router 初始化路由
fn init_router(mut router: Router) -> Router {
    router = router.route("/", get(root));
    router = ctrl::admin::init_router(router);
    router = ctrl::article::init_router(router);
    router  = router.layer(middleware::from_fn(ctrl::interceptor::print_request_body));
    return router;
}

async fn root() -> &'static str {
    "Hello, World!"
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