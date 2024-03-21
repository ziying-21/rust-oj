use crate::USER_LIST;
use actix_web::{get, HttpResponse, Responder};
#[get("/users")]
pub async fn get_users() -> impl Responder {
    let lock = USER_LIST.lock().unwrap();
    let u_l = lock.clone();
    drop(lock);
    HttpResponse::Ok().json(u_l)
}
