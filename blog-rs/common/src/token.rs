use log::debug;

pub fn generate_user_token(user_name: String) -> String {
    let u = plier::uid::uid_v4();
    let t = plier::md::sha256(format!("{}{}",u, user_name));
    return format!("ut{}", t);
}

