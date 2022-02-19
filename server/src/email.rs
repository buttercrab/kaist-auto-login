use crate::login::login;
use serde::{Deserialize, Serialize};

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

pub async fn get_code(_id: String, _email: String) -> GetCode {
    GetCode::Timeout
}
