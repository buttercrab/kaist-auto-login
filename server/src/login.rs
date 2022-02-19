use std::time::Duration;

use thirtyfour::prelude::*;
use tokio::time::sleep;

use crate::Config;

pub async fn login(id: String, pw: String) -> WebDriverResult<bool> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()?;
    let driver = WebDriver::new(Config::chrome_driver_url(), &caps).await?;

    // Navigate to https://wikipedia.org.
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

    // Find element from element.
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
