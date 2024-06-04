
use std::sync::RwLock;
use once_cell::sync::OnceCell;
use plier::rds;

static RDS: OnceCell<RwLock<rds::RDS>> = OnceCell::new();

pub async fn initialize_global_object(r: rds::RDS) {
    let _ = RDS.set(RwLock::new(r));
}

pub async fn get_user_by_token(user_token: String) -> String {
    let rd = RDS.get().expect("RDS is not initialized").read().unwrap();
    let res = rd.get_string(&format!("UT:{}", user_token)).await;

    if res.is_err() {
        return "".to_string();
    }

    return res.unwrap();
}