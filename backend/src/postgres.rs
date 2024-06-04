use std::env::var;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

fn create_config() -> Config {
    let mut cfg = Config::new();

    if let Ok(host) = var("PG_HOST") {
        cfg.host = Some(host);
    }
    if let Ok(dbname) = var("PG_DBNAME") {
        cfg.dbname = Some(dbname);
    }
    if let Ok(user) = var("PG_USER") {
        cfg.user = Some(user);
    }
    if let Ok(password) = var("PG_PASSWORD") {
        cfg.password = Some(password);
    }

    cfg
}

pub fn create_pool() -> Pool {
    create_config()
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Couldn't create postgres pool")
}
