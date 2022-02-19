use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::timeout;

use crate::login::login;
use crate::Config;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GenerateEmail {
    Success { email: String },
    LoginFail,
    EmailGenerationFail,
}

pub async fn generate_email(id: String, pw: String) -> GenerateEmail {
    let _ = login(id, pw);
    GenerateEmail::LoginFail
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GetCode {
    Success { code: String },
    NoAccount,
    Timeout,
}

async fn get_code_impl(_id: String, _email: String) -> GetCode {
    GetCode::NoAccount
}

pub async fn get_code(id: String, email: String) -> GetCode {
    timeout(
        Duration::from_secs(Config::get_code_timeout()),
        get_code_impl(id, email),
    )
    .await
    .unwrap_or(GetCode::Timeout)
}
