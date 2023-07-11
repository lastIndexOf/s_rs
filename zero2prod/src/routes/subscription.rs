use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SubscribeFormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscribeFormData>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match query!(
        r###"INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)"###,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
