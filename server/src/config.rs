use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use log::error;
use once_cell::sync::OnceCell;
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

fn default_get_code_timeout() -> u64 {
    10 * 60
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    host: IpAddr,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_chrome_driver_url")]
    chrome_driver_url: String,
    #[serde(default = "default_get_code_timeout")]
    get_code_timeout: u64,
}

impl Config {
    pub fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();

        CONFIG.get_or_init(|| match envy::from_env() {
            Ok(config) => config,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        })
    }

    pub fn host() -> IpAddr {
        Config::global().host
    }

    pub fn port() -> u16 {
        Config::global().port
    }

    pub fn address() -> SocketAddr {
        SocketAddr::from((Config::host(), Config::port()))
    }

    pub fn chrome_driver_url() -> &'static str {
        &Config::global().chrome_driver_url
    }

    pub fn get_code_timeout() -> u64 {
        Config::global().get_code_timeout
    }
}
