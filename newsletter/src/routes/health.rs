use actix_web::HttpResponse;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
