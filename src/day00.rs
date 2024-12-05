use actix_web::{get, http::StatusCode, web, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_hello_bird).service(get_seek);
}

#[get("/")]
async fn get_hello_bird() -> impl Responder {
    "Hello, bird!"
}

#[get("/-1/seek")]
async fn get_seek() -> impl Responder {
    web::Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4")
        .using_status_code(StatusCode::FOUND)
}
