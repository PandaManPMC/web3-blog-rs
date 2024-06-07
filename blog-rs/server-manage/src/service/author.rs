use i_dao::i_mysql;
use r2d2_mysql::mysql::Transaction;
use base::dao::blog_author_dao;
use base::model::blog_author::BlogAuthorModel;

pub async fn find_by_user_name(user_name: String) -> mysql::Result<Option<BlogAuthorModel>, Box<dyn std::error::Error>> {
    let mut call = | tx:&mut Transaction |  -> mysql::Result<Option<BlogAuthorModel>, Box<dyn std::error::Error>>  {
        let result = blog_author_dao::find_by_user_name(tx, user_name.clone());
        return Ok(result?);
    };
    // return Ok(i_mysql::start_tx(&crate::service::get_data_source_key().await, &mut call)?);
    return Ok(i_mysql::start_tx("mysql1", &mut call)?);
}

