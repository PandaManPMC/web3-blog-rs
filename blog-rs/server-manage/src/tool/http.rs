use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteVerifyV2Rsp {
    pub success: bool,
    #[serde(default)]
    #[serde(rename = "challenge_ts")]
    pub challenge_ts: String,
    #[serde(default)]
    pub hostname: String,
    #[serde(default)]
    #[serde(rename = "error-codes")]
    pub error_codes: Option<Vec<String>>,
}

pub async fn verify_re_captcha_token_v2(token: String, secret: String) -> Result<SiteVerifyV2Rsp, String> {
    let url = "https://www.recaptcha.net/recaptcha/api/siteverify";
    let params = [("secret", secret), ("response", token)];

    let client = reqwest::Client::new();
    let res = client.post(url)
        .form(&params)
        .send()
        .await;

    if res.is_err() {
        return Err(res.err().unwrap().to_string())
    }

    let r = res.unwrap();
    let body = r.text().await.unwrap();
    let sv: SiteVerifyV2Rsp = serde_json::from_str(&body).unwrap();
    Ok(sv)
}
