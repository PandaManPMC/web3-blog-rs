use i_dao::tok::i_mysql;
use r2d2_mysql::mysql::Transaction;
use base::dao::blog_author_dao;
use base::model::blog_author::BlogAuthorModel;

pub async fn find_by_user_name(user_name: String) -> Result<Option<BlogAuthorModel>, String> {
    let mut call = | tx:&mut Transaction |  -> mysql::Result<Option<BlogAuthorModel>, Box<dyn std::error::Error>>  {
        let result = blog_author_dao::find_by_user_name(tx, user_name.clone());
        return Ok(result?);
    };
    // return Ok(i_mysql::start_tx(&crate::service::get_data_source_key().await, &mut call)?);
    let res = i_mysql::start_tx("mysql_db1", &mut call).await;

    if res.is_err() {
        return Err(res.err().expect("err").to_string());
    }

    return Ok(res.unwrap());
}
