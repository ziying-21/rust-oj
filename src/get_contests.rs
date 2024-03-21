use actix_web::{get, HttpResponse, Responder};

use crate::CONTEST_LIST;

#[get("/contests")]
pub async fn get_contests() -> impl Responder {
    let lock = CONTEST_LIST.lock().unwrap();
    let contests_list = lock.clone();
    drop(lock);
    HttpResponse::Ok().json(contests_list)
}
