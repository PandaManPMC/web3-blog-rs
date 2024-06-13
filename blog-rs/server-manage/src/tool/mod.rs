use i_dao::sql;
use log::info;

pub mod req;
pub mod http;
mod tool_test;

pub const X_USER_TOKEN: &str = "x-user-token";

pub const X_USER_ID: &str = "x-user-id";

pub const X_USER_NAME:&str = "x-user-name";

pub const X_USER_PEN_NAME:&str = "x-user-pen_name";

pub fn limit_min() -> [sql::Condition;2] {
    let page_index = sql::Condition::PageIndex(1);
    let page_size = sql::Condition::PageSize(1);
    let bc = [page_index, page_size ];
    return bc;
}

pub fn limit_max() -> [sql::Condition;2] {
    let page_index = sql::Condition::PageIndex(1);
    let page_size = sql::Condition::PageSize(1000);
    let bc = [page_index, page_size ];
    return bc;
}
