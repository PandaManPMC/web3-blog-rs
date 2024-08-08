use i_dao::tok::i_mysql;
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
use axum::extract::DefaultBodyLimit;
use tokio_cron_scheduler::{JobScheduler, Job};

mod ctrl;
mod service;
mod bean;
mod utils;
mod dao;

#[tokio::main]
async fn main() {
    let p = plier::files::get_current_dir_str();
    println!("{:?}", p);

    let mut conf_file_path = String::from("server-api/src/config.toml");
    let mut log_conf_path = String::from("server-api/src/log4rs.yaml");

    if cfg!(target_os = "linux") {
        conf_file_path = String::from("./config.toml");
        log_conf_path = String::from("./log4rs.yaml");
    }

    unsafe {
        let _res = configs::init(conf_file_path.clone());
        log4rs::init_file(log_conf_path.clone(), Default::default()).unwrap();
        info!("log4rs::init_file {:?}", log_conf_path.clone());

        info!("env = {}", configs::get_str("basics", "env"));
        info!("port = {}", configs::get_int("basics", "port"));

        init_mysql().await;
        init_rds().await;

        let mut app = Router::new();
        app = init_router(app);

        info!("server = {:?}", format!("0.0.0.0:{}", configs::get_int("basics", "port")));

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", configs::get_int("basics", "port"))).await.unwrap();
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    }

}

async fn init_rds(){
    unsafe {
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
    router = ctrl::article::init_router(router);
    router = router.layer(middleware::from_fn(common::net::interceptor::error_handling));
    router = router.layer(middleware::from_fn(ctrl::interceptor::app));
    router = router.layer(DefaultBodyLimit::max(1 * 1024 * 1024));
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

    i_mysql::init(base::service::get_data_source_key().await, opts, configs::get_int("mysql_db1", "max_size") as u32, configs::get_int("mysql_db1", "max_idle") as u32).await;

    let conn = i_mysql::get_conn(&base::service::get_data_source_key().await).await;
    // let conn = i_mysql::get_conn(&service::get_data_source_key().await).await;

    if conn.is_err() {
        warn!("init_mysql {:?}", conn);
        panic!("init_mysql");
    }
}

async fn init_schedule() {
    let sched = JobScheduler::new().await.unwrap();

    // 每十分钟执行一次, 缓存作者
    sched.add(Job::new("0 0/10 * * * ?", |_, _| {
        let _ = Box::pin(async {
            service::blog::cache_author().await;
        });
    }).unwrap()).await.unwrap();


    // 缓存标签
    sched.add(Job::new("0 0/10 * * * ?",  |_, _| {
        let _ = Box::pin(async {
            service::blog::cache_label().await;
        });
    }).unwrap()).await.unwrap();


    // 开始调度
    sched.start().await.unwrap();

    // 让主任务运行一段时间以观察调度效果
    tokio::time::sleep(Duration::from_secs(60)).await;
}