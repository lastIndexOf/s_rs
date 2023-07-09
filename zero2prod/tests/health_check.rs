use std::net::TcpListener;

use anyhow::Result;
use reqwest::StatusCode;
use sqlx::{query, Connection, PgConnection};
use zero2prod::{configuration::get_configuration, startup::run};

/// 启动一个测试的 http server
/// 返回格式: xxx.x.x.x:port
fn spawn_app() -> Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let server = run(listener)?;
    tokio::spawn(server);
    Ok(addr.to_string())
}

#[tokio::test]
async fn health_check_works() -> Result<()> {
    let addr = spawn_app()?;

    let res = reqwest::Client::new()
        .get(format!("http://{}/health_check", addr))
        .send()
        .await?;

    assert!(res.status().is_success());
    assert_eq!(res.content_length(), Some(0));

    Ok(())
}

#[tokio::test]
async fn subscriptions_return_200_when_date_validate() -> Result<()> {
    let addr = spawn_app()?;

    let settings = get_configuration()?;
    let connection_string = settings.postgres.connection_string();
    let mut connection = PgConnection::connect(&connection_string).await?;

    let body = "name=kgaikj2cu&email=kgaikj2cu@icloud.com";
    let res = reqwest::Client::new()
        .post(format!("http://{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    assert_eq!(res.status().as_u16(), StatusCode::OK);

    let saved = query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await?;

    assert_eq!(saved.name, "zhengfankai");
    assert_eq!(saved.email, "kgaikj2cu@icloud.com");

    Ok(())
}

#[tokio::test]
async fn subscriptions_return_400_when_date_miss() -> Result<()> {
    let addr = spawn_app()?;

    let bodies = ["email=kgaikj2cu@icloud.com", "name=kgaikj2cu", ""];

    for body in bodies {
        let res = reqwest::Client::new()
            .post(format!("http://{}/subscriptions", addr))
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
