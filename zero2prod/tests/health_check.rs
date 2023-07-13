use std::net::TcpListener;

use anyhow::Result;
use reqwest::StatusCode;
use sqlx::{migrate, query, Connection, Executor, PgConnection, PgPool};
use zero2prod::{
    configuration::{get_configuration, PostgresSettings, Settings},
    startup::run,
};

struct TestApp {
    address: String,
    pool: PgPool,
}

/// 启动一个测试的 http server
/// 返回格式: xxx.x.x.x:port
async fn spawn_app() -> Result<TestApp> {
    let mut settings = get_configuration()?;
    let test_app_db_name = uuid::Uuid::new_v4().to_string();
    settings.postgres.db_name = test_app_db_name.clone();
    let pool = configure_database(&settings.postgres).await?;

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    let server = run(listener, pool.clone())?;

    tokio::spawn(server);
    Ok(TestApp {
        address: addr.to_string(),
        pool,
    })
}

async fn configure_database(database_settings: &PostgresSettings) -> Result<PgPool> {
    let mut connection =
        PgConnection::connect(&database_settings.connection_without_db_string()).await?;

    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, database_settings.db_name).as_str())
        .await?;

    let pool = PgPool::connect(&database_settings.connection_string()).await?;

    migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[tokio::test]
async fn health_check_works() -> Result<()> {
    let test_app = spawn_app().await?;

    let res = reqwest::Client::new()
        .get(format!("http://{}/health_check", test_app.address))
        .send()
        .await?;

    assert!(res.status().is_success());
    assert_eq!(res.content_length(), Some(0));

    Ok(())
}

#[tokio::test]
async fn subscriptions_return_200_when_date_validate() -> Result<()> {
    let test_app = spawn_app().await?;

    let body = "name=kgaikj2cu&email=kgaikj2cu@icloud.com";
    let res = reqwest::Client::new()
        .post(format!("http://{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    assert_eq!(res.status().as_u16(), StatusCode::OK);

    let saved = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.pool)
        .await?;

    assert_eq!(saved.name, "kgaikj2cu");
    assert_eq!(saved.email, "kgaikj2cu@icloud.com");

    Ok(())
}

#[tokio::test]
async fn subscriptions_return_400_when_date_miss() -> Result<()> {
    let test_app = spawn_app().await?;

    let bodies = ["email=kgaikj2cu@icloud.com", "name=kgaikj2cu", ""];

    for body in bodies {
        let res = reqwest::Client::new()
            .post(format!("http://{}/subscriptions", test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await?;

        assert_eq!(
            res.status().as_u16(),
            StatusCode::BAD_REQUEST,
            "name or email missing will return 400 error"
        );
    }

    Ok(())
}
