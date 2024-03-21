use actix_web::{options, HttpResponse, Responder};

#[options("/jobs")]
pub async fn options() -> impl Responder {
    HttpResponse::Ok()
}
