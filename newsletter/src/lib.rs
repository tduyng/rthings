use actix_web::{web, App, HttpResponse, HttpServer};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .bind("127.0.0.1:2024")?
        .run()
        .await
}
