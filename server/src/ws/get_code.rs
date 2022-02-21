use std::thread::sleep;
use std::time::Duration;

use chrono::Local;
use imap_proto::{MessageSection, SectionPath};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;

use crate::{conf, database};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GetCodeError {
    MailServerError,
    InternalError,
    NoAccount,
    Timeout,
}

impl From<imap::Error> for GetCodeError {
    fn from(_: imap::Error) -> Self {
        GetCodeError::MailServerError
    }
}

impl From<JoinError> for GetCodeError {
    fn from(_: JoinError) -> Self {
        GetCodeError::InternalError
    }
}

async fn get_email_account(id: &str) -> Result<(String, String), GetCodeError> {
    let client = database::get_client().await;
    let stmt = client
        .prepare_cached("select email_id, email_pw from account where id = $1")
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&id]).await.unwrap();

    if !rows.is_empty() {
        Ok((rows[0].get(0), rows[0].get(1)))
    } else {
        Err(GetCodeError::NoAccount)
    }
}

pub async fn get_code(id: String) -> Result<String, GetCodeError> {
    let (email_id, email_pw) = get_email_account(&id).await?;
    let timeout = conf::get_code_timeout_dur();
    let start = Local::now();

    // Since the IMAP operation contains lots of IO blocking,
    // we can use spawn blocking. There are alternatives like
    // async-imap or tokio-imap, but they are not very good.
    tokio::task::spawn_blocking(move || {
        let client =
            imap::ClientBuilder::new(conf::imap_domain(), conf::imap_port()).native_tls()?;
        let mut session = client.login(email_id, email_pw).map_err(|e| e.0)?;
        let total = session.select("INBOX")?.exists;
        let mut checked = std::cmp::max(0, total - 5);

        while Local::now() - start <= timeout {
            let total = session.select("INBOX")?.exists;
            if checked < total {
                let msg = session.fetch(
                    format!("{}:{total}", checked + 1),
                    "(BODY.PEEK[HEADER.FIELDS (\"FROM\")] INTERNALDATE BODY.PEEK[TEXT])",
                )?;

                for (idx, i) in msg.iter().enumerate().rev() {
                    let from = std::str::from_utf8(
                        i.section(&SectionPath::Full(MessageSection::Header))
                            .unwrap(),
                    )
                    .unwrap();
                    let time = i.internal_date().unwrap();
                    let body = std::str::from_utf8(
                        i.section(&SectionPath::Full(MessageSection::Text)).unwrap(),
                    )
                    .unwrap();

                    let re = Regex::new(r#"From: ".*" <iamps@kaist\.ac\.kr>"#).unwrap();
                    if re.is_match(from) && time >= start {
                        let re = Regex::new(r#"<span id=3D"sendValue1">([0-9]+)</span>"#).unwrap();
                        let code = re
                            .captures(body)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .to_string();

                        session.mv(format!("{}", idx + checked as usize + 1), "Trash")?;
                        session.delete("Trash")?;
                        session.logout()?;

                        return Ok(code);
                    }
                }

                checked = total;
            }

            static SLEEP_DURATION: Duration = Duration::from_millis(100);
            sleep(SLEEP_DURATION);
        }

        session.logout()?;

        Err(GetCodeError::Timeout)
    })
    .await?
}
