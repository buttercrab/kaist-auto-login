use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use chrono::Duration;
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Pool, Runtime};
use log::error;
use once_cell::sync::OnceCell;
use rand::{thread_rng, Rng};
use serde::Deserialize;

fn default_host() -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
}

fn default_port() -> u16 {
    80
}

fn default_chrome_driver_url() -> String {
    "http://localhost:4444".to_string()
}

fn default_get_code_timeout() -> i64 {
    10 * 60
}

fn default_imap_port() -> u16 {
    993
}

fn default_email_id_charset() -> Vec<u8> {
    b"0123456789abcdefghijklmnopqrstuvwxyz".to_vec()
}

fn default_email_id_length() -> usize {
    6
}

fn default_email_pw_charset() -> Vec<u8> {
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~".to_vec()
}

fn default_email_pw_length() -> usize {
    10
}

#[derive(Debug, Clone, Deserialize)]
struct Config {
    #[serde(default = "default_host")]
    host: IpAddr,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_chrome_driver_url")]
    chrome_driver_url: String,
    #[serde(default = "default_get_code_timeout")]
    get_code_timeout: i64,
    imap_domain: String,
    #[serde(default = "default_imap_port")]
    imap_port: u16,
    mailcow_url: String,
    mailcow_api_key: String,
    new_email_domain: String,
    #[serde(default = "default_email_id_charset")]
    email_id_charset: Vec<u8>,
    #[serde(default = "default_email_id_length")]
    email_id_length: usize,
    #[serde(default = "default_email_pw_charset")]
    email_pw_charset: Vec<u8>,
    #[serde(default = "default_email_pw_length")]
    email_pw_length: usize,
    #[serde(default)]
    postgres: deadpool_postgres::Config,
}

fn global() -> &'static Config {
    static CONFIG: OnceCell<Config> = OnceCell::new();

    CONFIG.get_or_init(|| {
        let builder = config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .add_source(config::File::with_name("config"));

        match builder.build().and_then(|b| b.try_deserialize()) {
            Ok(config) => config,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        }
    })
}

pub fn host() -> IpAddr {
    global().host
}

pub fn port() -> u16 {
    global().port
}

pub fn address() -> SocketAddr {
    SocketAddr::from((host(), port()))
}

pub fn chrome_driver_url() -> &'static str {
    &global().chrome_driver_url
}

pub fn get_code_timeout() -> i64 {
    global().get_code_timeout
}

pub fn get_code_timeout_dur() -> Duration {
    Duration::seconds(get_code_timeout())
}

pub fn imap_domain() -> &'static str {
    &global().imap_domain
}

pub fn imap_port() -> u16 {
    global().imap_port
}

pub fn mailcow_url() -> &'static str {
    &global().mailcow_url
}

pub fn mailcow_api_key() -> &'static str {
    &global().mailcow_api_key
}

pub fn new_email_domain() -> &'static str {
    &global().new_email_domain
}

pub fn email_id_charset() -> &'static [u8] {
    &global().email_id_charset
}

pub fn email_id_length() -> usize {
    global().email_id_length
}

pub fn email_pw_charset() -> &'static [u8] {
    &global().email_pw_charset
}

pub fn email_pw_length() -> usize {
    global().email_pw_length
}

pub fn random_email_id() -> String {
    let mut rng = thread_rng();
    let charset = email_id_charset();

    (0..email_id_length())
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn random_email_pw() -> String {
    let mut rng = thread_rng();
    let charset = email_pw_charset();

    (0..email_pw_length())
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn get_postgres() -> &'static deadpool_postgres::Config {
    &global().postgres
}

pub fn create_postgres_pool() -> Pool {
    get_postgres()
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .unwrap()
}
