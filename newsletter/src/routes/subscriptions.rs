// use actix_web::{web, HttpResponse};
// use sqlx::PgPool;
// use chrono::Utc;
// use uuid::Uuid;

use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code)]
// pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
pub async fn subscribe() -> HttpResponse {
    // match sqlx::query!(
    //     r#"
    // INSERT INTO subscriptions (id, email, name, subscribed_at)
    // VALUES ($1, $2, $3, $4)
    //         "#,
    //     Uuid::new_v4(),
    //     form.email,
    //     form.name,
    //     Utc::now()
    // )
    // .execute(pool.as_ref())
    // .await
    // {
    //     Ok(_) => HttpResponse::Ok().finish(),
    //     Err(e) => {
    //         println!("Failed to execute query: {}", e);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }
    HttpResponse::Ok().finish()
}
