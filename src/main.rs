mod day00;
mod day02;
mod day05;
mod day09;
mod day12;
mod day16;
mod day19;

use actix_web::{error, web, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!().run(&pool).await.unwrap();

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.configure(day00::configure)
            .configure(day02::configure)
            .configure(day05::configure)
            .configure(day09::configure)
            .configure(day12::configure)
            .configure(day16::configure)
            .configure(day19::configure)
            .app_data(web::Data::new(pool))
            .app_data(web::PathConfig::default().error_handler(|err, _| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().into()).into()
            }));
    };

    Ok(config.into())
}
