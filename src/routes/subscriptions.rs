use actix_web::{web, Responder, HttpResponse, post};
use serde::Deserialize;

#[post("/subscriptions")]
async fn subscribe(form: web::Form<Subscriber>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize, Debug)]
struct Subscriber {
    name: String,
    email: String,
}
