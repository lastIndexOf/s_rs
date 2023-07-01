use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SubscribeFormData {
    name: String,
    email: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<SubscribeFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
