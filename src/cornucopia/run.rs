use deadpool_postgres::{Config, CreatePoolError, Object, Runtime};
use std::env;
use time::OffsetDateTime;
use tokio_postgres::NoTls;
use uuid::Uuid;

use super::queries::queries::demos::{demos, insert_demo};

pub async fn create_client() -> Result<Object, CreatePoolError> {
    let (host, port, username, password, database) = (
        env::var("postgres_host").expect("Env postgres_host not found"),
        env::var("postgres_port").expect("Env postgres_port not found"),
        env::var("postgres_username").expect("Env postgres_username not found"),
        env::var("postgres_password").expect("Env postgres_password not found"),
        env::var("postgres_database").expect("Env postgres_database not found"),
    );
    let mut cfg = Config::new();
    cfg.host = Some(host);
    cfg.port = Some(port.parse().unwrap());
    cfg.user = Some(username);
    cfg.password = Some(password);
    cfg.dbname = Some(database);
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let client = pool.get().await.unwrap();
    Ok(client)
}

pub async fn run() {
    let client = create_client().await.unwrap();
    let demo = insert_demo()
        .bind(
            &client,
            &Uuid::new_v4(),
            &"test",
            &OffsetDateTime::now_utc(),
        )
        .one()
        .await
        .unwrap();
    dbg!(demo);
    let demos = demos().bind(&client).all().await.unwrap();
    dbg!(demos);
}
