use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("0.0.0.0:8083")?)?.await
}
