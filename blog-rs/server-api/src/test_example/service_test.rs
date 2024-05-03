use i_dao::{i_mysql, sql};
use log::{debug, trace};
use r2d2_mysql::mysql::OptsBuilder;
use std::time::Duration;
use std::collections::HashMap;
use std::any::Any;

fn init_mysql() {
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

#[cfg(test)]
mod test_service {
    use log::{info, warn};
    use super::*;

    #[test]
    fn test_add(){
        init_mysql();
        let mut label = base::model::blog_label::BlogLabelModel::new("Rust".to_string(), 1);
        let res = base::service::blog_label_sve::add(&mut label);
        println!("{:?}", res);
        println!("{:?}", label);
    }

    #[test]
    fn test_add_batch(){
        init_mysql();
        // let mut label1 = base::model::blog_label::BlogLabelModel::new("Golang".to_string(), 1);
        // let mut label2 = base::model::blog_label::BlogLabelModel::new("Java".to_string(), 1);
        let mut label1 = base::model::blog_label::BlogLabelModel::new("JavaScript".to_string(), 1);
        let mut label2 = base::model::blog_label::BlogLabelModel::new("Solidity".to_string(), 1);
        let mut lst = vec![&mut label1,&mut label2];

        let res = base::service::blog_label_sve::add_batch(&mut lst);
        println!("{:?}", res);
        println!("{:?}", lst);
    }

    #[test]
    fn test_find_by_name() {
        init_mysql();
        let res = base::service::blog_label_sve::find_by_label_name("Golang".to_string());
        println!("{:?}", res);

        match res {
            Ok(v) => {
                println!("{:?}", v);
                match v {
                    Some(a) => {
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

    #[test]
    fn test_find() {
        init_mysql();
        println!("{:?}", base::model::blog_label::FIELDS);
        let res = base::service::blog_label_sve::find_by_id(3);
        println!("{:?}", res);

        match res {
            Ok(v) => {
                println!("{:?}", v);
                match v {
                    Some(a) => {
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

    #[test]
    fn test_update() {
        init_mysql();
        let res = base::service::blog_label_sve::find_by_id(5);
        println!("{:?}", res);
        if res.is_err() {
            println!("查询错误");
            return;
        }

        if let Ok(None) = res {
            println!("未查询到");
            return;
        }

        let mut m = res.unwrap().unwrap();

        m.state = 2;
        let res2 = base::service::blog_label_sve::update_by_id(&mut m);
        println!("{:?}", res2);
    }

    #[test]
    fn test_query_list() {
        init_mysql();
        let mut params:HashMap<String, Box<dyn Any>> = HashMap::new();
        params.insert(String::from(format!("{}state", sql::LT)), Box::new(2));

        let page_index = sql::Condition::PageIndex(1);
        let page_size = sql::Condition::PageSize(3);
        let asc = sql::Condition::OrderByAESOrDESC(1);

        let bc = [page_index, page_size, asc, ];

        let res = base::service::blog_label_sve::query_list(&params, &bc);
        if res.is_err() {
            println!("出现异常 {:?}", res);
            return;
        }
        let lst = res.unwrap();
        println!("查询到={:?}条", lst.len());


        for v in &lst {
            println!("{:?}", v);
        }

        let res2 = base::service::blog_label_sve::query_count(&params, &bc);
        println!("{:?}", res2.unwrap());


    }

}