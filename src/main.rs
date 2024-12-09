mod day00;
mod day02;
mod day05;
mod day09;

use actix_web::web;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.configure(day00::configure)
            .configure(day02::configure)
            .configure(day05::configure)
            .configure(day09::configure);
    };

    Ok(config.into())
}
