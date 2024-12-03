mod day00;
mod day02;

use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

type ShuttleResult<T> = Result<T, Box<dyn std::error::Error>>;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(day00::configure).configure(day02::configure);
    };

    Ok(config.into())
}
