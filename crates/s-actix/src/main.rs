use std::{io::ErrorKind, time::Duration};

use actix_web::{
    get, guard, post,
    web::{self, Data, Path, Query},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

struct AppData {
    app_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<i32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/error")]
async fn post_error() -> Result<impl Responder, std::io::Error> {
    Err::<String, std::io::Error>(std::io::Error::new(ErrorKind::AddrInUse, "ss"))
}

#[post("/echo")]
async fn echo(req_body: String, app_data: Data<AppData>) -> String {
    format!("{}: {}", app_data.app_name, req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn app_index() -> impl Responder {
    HttpResponse::Ok().body("<html>Hello World</html>")
}

#[get("/user/{user_id}/{girlfriend}")]
async fn gf_handler(query: Query<UserQuery>, path: Path<(String, String)>) -> String {
    let (user_id, girlfriend) = path.into_inner();
    let age = query.age.or(Some(0)).unwrap();
    format!("my age = {age}, {}'s girlfriend is {}", user_id, girlfriend)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppData {
                app_name: String::from("Actix Web"),
            }))
            .service(
                web::scope("/app")
                    .guard(guard::Get())
                    .route("/index", web::get().to(app_index)),
            )
            .service(hello)
            .service(echo)
            .service(gf_handler)
            .service(post_error)
            .service(web::resource("/test/resource").route(web::route().to(manual_hello)))
            .service(web::scope("/test").guard(guard::Post()).service(echo))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8001")?
    .workers(4)
    .keep_alive(Duration::from_secs(10))
    // .shutdown_timeout(30)
    // .disable_signals()
    .run()
    .await
}
