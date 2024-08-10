use std::collections::HashMap;
use i_dao::sql;
use crate::service::ADVERTISE_LIST;
use crate::utils;

/// 缓存广告
pub async fn cache_advertise() {
    tracing::info!("cache_advertise");

    let mut params:HashMap<String, sql::Params> = HashMap::new();
    params.insert(String::from("state"), sql::Params::UInteger8(1));

    let result = base::service::advertise_info_sve::query_list(&params, &utils::limit_max()).await;
    if result.is_err() {
        tracing::warn!("{:?}", result);
        return
    }
    let lst = result.unwrap();

    let mut cache = ADVERTISE_LIST.write().await;
    for obj in lst {
        tracing::info!("cache_advertise {:?}={:?}", obj.id,obj.title);
        cache.insert(obj.id, obj);
    }
}