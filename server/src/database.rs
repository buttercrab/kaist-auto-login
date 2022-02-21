use deadpool_postgres::{Client, Pool};
use once_cell::sync::OnceCell;

use crate::conf::create_postgres_pool;

static POOL: OnceCell<Pool> = OnceCell::new();
static INIT_SQL: &str = include_str!("../sql/init.sql");

fn get_pool() -> &'static Pool {
    POOL.get_or_init(create_postgres_pool)
}

pub async fn get_client() -> Client {
    get_pool().get().await.unwrap()
}

pub async fn init() {
    let client = get_client().await;
    let stmt = client.prepare(INIT_SQL).await.unwrap();
    let _ = client.execute(&stmt, &[]).await.unwrap();
}
