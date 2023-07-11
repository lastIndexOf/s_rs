use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_configuration().expect("get configuration error");
    let connection_string = settings.postgres.connection_string();

    let connection = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres.");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", settings.port))?;

    run(listener, connection)?.await
}
