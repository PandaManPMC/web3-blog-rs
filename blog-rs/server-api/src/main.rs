use i_dao::i_mysql;
use log::{debug, trace};
use r2d2_mysql::mysql::OptsBuilder;
use env_logger::{Env, init};
use std::time::Duration;

mod service;

fn init_mysql() {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    base::service::set_date_source_key(String::from("mysql_db1"));
    debug!("{:?}", base::service::get_data_source_key());

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("34.150.97.232"))
        .user(Some("rs_blog"))
        .pass(Some("abcd#1890dda1"))
        .db_name(Some("rs_blog"))
        .tcp_port(23306)
        .tcp_connect_timeout(Some(Duration::from_secs(30)));

    i_mysql::init(base::service::get_data_source_key(), opts, 200, 5);
    let conn = i_mysql::get_conn(&base::service::get_data_source_key());
    trace!("{:?}", conn);
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", base::model::blog_view::FIELDS);
    println!("{:?}", base::model::blog_article::FIELDS);

    service::blog::test();

    println!("{:?}", base::model::blog_classes::FIELDS);

    init_mysql();

    let res = base::service::blog_article_sve::find_by_id(1);
    println!("{:?}", res);

    match res  {
        Ok(v) => {
            println!("{:?}", v);
            match v {
                Some(a) =>{
                    println!("{:?}", a);
                }
                _ => {
                    println!("未找到");
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
        },
    }

}
