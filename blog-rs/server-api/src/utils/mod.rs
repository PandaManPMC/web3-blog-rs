use i_dao::sql;

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
