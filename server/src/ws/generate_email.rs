use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use thirtyfour::prelude::WebDriverResult;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use tokio::time::sleep;

use crate::{conf, database};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GenerateEmailError {
    LoginFail,
    EmailGenerationFail,
}

async fn login(id: &str, pw: &str) -> WebDriverResult<bool> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    let driver = WebDriver::new(conf::chrome_driver_url(), &caps).await?;

    driver.delete_all_cookies().await?;
    driver.get("https://iam2.kaist.ac.kr").await?;

    let id_ipt = driver.find_element(By::Id("IdInput")).await?;
    id_ipt.send_keys(id).await?;

    let pw_btn = driver
        .find_element(By::XPath(
            "/html/body/div/div/div[2]/div/div/fieldset/ul/li[2]/input[2]",
        ))
        .await?;
    pw_btn.click().await?;

    let pw_ipt = driver.find_element(By::Id("passwordInput")).await?;
    pw_ipt.send_keys(pw).await?;

    let login = driver.find_element(By::ClassName("loginbtn")).await?;
    login.click().await?;

    let ret = loop {
        let alert = driver.switch_to().alert();

        if alert.text().await.is_ok() {
            break false;
        } else if driver.current_url().await? == "https://iam2.kaist.ac.kr/#/checkOtp" {
            break true;
        }

        sleep(Duration::from_millis(10)).await;
    };

    driver.quit().await?;

    Ok(ret)
}

async fn random_email_id() -> String {
    loop {
        let id = conf::random_email_id();
        let client = database::get_client().await;
        let stmt = client
            .prepare_cached("select no from account where id = $1")
            .await
            .unwrap();
        let rows = client.query(&stmt, &[&id]).await.unwrap();

        if rows.is_empty() {
            break id;
        }
    }
}

async fn add_account(email_id: &str, id: &str, email_pw: &str) -> Result<(), GenerateEmailError> {
    let client = database::get_client().await;
    let stmt = client
        .prepare_cached("insert into account (email_id, id, email_pw) values ($1, $2, $3)")
        .await
        .unwrap();
    let rows = client
        .execute(&stmt, &[&email_id, &id, &email_pw])
        .await
        .unwrap();

    if rows == 1 {
        Ok(())
    } else {
        Err(GenerateEmailError::EmailGenerationFail)
    }
}

pub async fn generate_email(id: String, pw: String) -> Result<String, GenerateEmailError> {
    if !login(&id, &pw).await.unwrap_or(false) {
        return Err(GenerateEmailError::LoginFail);
    }

    let email_id = random_email_id().await;
    let email_pw = conf::random_email_pw();

    let client = reqwest::Client::new();
    let res = client
        .post(conf::mailcow_url())
        .header("X-API-Key", conf::mailcow_api_key())
        .body(
            serde_json::to_string(&json!({
                "active": "1",
                "domain": conf::new_email_domain(),
                "local_part": email_id,
                "name": format!("kaist-authbot-{}", email_id),
                "password": email_pw,
                "password2": email_pw,
                "quota": "10",
                "force_pw_update": "0",
                "tls_enforce_in": "1",
                "tls_enforce_out": "1"
            }))
            .unwrap(),
        )
        .send()
        .await
        .map_err(|_| GenerateEmailError::EmailGenerationFail)?;

    if res.status().is_success() {
        add_account(&email_id, &id, &email_pw).await?;
        Ok(email_id)
    } else {
        Err(GenerateEmailError::EmailGenerationFail)
    }
}
