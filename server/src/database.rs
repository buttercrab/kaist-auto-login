use std::time::Duration;

use deadpool_postgres::tokio_postgres::Error;
use deadpool_postgres::{Client, Manager, Pool};
use log::{error, info};
use once_cell::sync::OnceCell;
use tokio::time::sleep;

use crate::conf::create_postgres_pool;

static POOL: OnceCell<Pool> = OnceCell::new();
static INIT_SQL: &str = include_str!("../sql/init.sql");

fn get_pool() -> &'static Pool {
    POOL.get_or_init(create_postgres_pool)
}

pub async fn get_client() -> Client {
    loop {
        match get_pool().get().await {
            Ok(client) => break client,
            Err(e) => {
                error!("connecting database failed: {}", e);
                info!("reconnecting in 5 secs...");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

pub async fn init() {
    get_client().await.batch_execute(INIT_SQL).await.unwrap();
}
