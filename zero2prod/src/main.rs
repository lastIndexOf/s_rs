use std::net::TcpListener;

use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_configuration().expect("get configuration error");
    run(TcpListener::bind(format!("0.0.0.0:{}", settings.port))?)?.await
}
