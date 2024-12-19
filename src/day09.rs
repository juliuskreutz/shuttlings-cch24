use std::{sync::LazyLock, time::Duration};

use actix_web::{http, post, web, HttpRequest, HttpResponse};
use shuttle_runtime::tokio::sync::Mutex;

static STATE: LazyLock<web::Data<State>> = LazyLock::new(Default::default);

struct State {
    rate_limiter: Mutex<leaky_bucket::RateLimiter>,
}

fn rate_limiter() -> leaky_bucket::RateLimiter {
    leaky_bucket::Builder::default()
        .initial(5)
        .interval(Duration::from_secs(1))
        .max(5)
        .build()
}

impl Default for State {
    fn default() -> Self {
        let rate_limiter = Mutex::new(rate_limiter());
        Self { rate_limiter }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let state = STATE.clone();

    cfg.service(post_milk).service(post_refill).app_data(state);
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum Unit {
    Liters(f32),
    Gallons(f32),
    Litres(f32),
    Pints(f32),
}

impl Unit {
    fn convert(self) -> Unit {
        match self {
            Unit::Liters(liters) => {
                let gallons = liters / 3.785_411_8;
                Unit::Gallons(gallons)
            }
            Unit::Gallons(gallons) => {
                let liters = gallons * 3.785_411_8;
                Unit::Liters(liters)
            }
            Unit::Litres(litres) => {
                let pints = litres * 1.759_754;
                Unit::Pints(pints)
            }
            Unit::Pints(pints) => {
                let litres = pints / 1.759_754;
                Unit::Litres(litres)
            }
        }
    }
}

#[post("/9/milk")]
async fn post_milk(
    req: HttpRequest,
    state: web::Data<State>,
    unit: Option<web::Json<Unit>>,
) -> HttpResponse {
    let content_type = req.headers().get(http::header::CONTENT_TYPE);

    let json = matches!(
        content_type.map(|c| c.to_str()),
        Some(Ok("application/json"))
    );

    if !state.rate_limiter.lock().await.try_acquire(1) {
        HttpResponse::TooManyRequests().body("No milk available\n")
    } else if !json {
        HttpResponse::Ok().body("Milk withdrawn\n")
    } else {
        match unit.map(|j| j.into_inner()) {
            Some(unit) => {
                let unit = unit.convert();

                HttpResponse::Ok().json(unit)
            }
            _ => HttpResponse::BadRequest().finish(),
        }
    }
}

#[post("/9/refill")]
async fn post_refill(state: web::Data<State>) -> HttpResponse {
    *state.rate_limiter.lock().await = rate_limiter();

    HttpResponse::Ok().finish()
}
