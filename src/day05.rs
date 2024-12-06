use actix_web::{http, post, web, HttpRequest, HttpResponse, Responder};

use crate::ShuttleResult;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(post_manifest);
}

#[derive(Default, serde::Deserialize)]
struct Metadata {
    #[serde(default)]
    orders: Vec<Order>,
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
struct Order {
    item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    quantity: Option<usize>,
}

#[post("/5/manifest")]
async fn post_manifest(req: HttpRequest, text: String) -> ShuttleResult<impl Responder> {
    let Some(content_type) = req.headers().get(http::header::CONTENT_TYPE) else {
        return Ok(HttpResponse::UnsupportedMediaType().finish());
    };

    let Some(manifest) = (match content_type.to_str()? {
        "application/toml" => toml::from_str::<cargo_manifest::Manifest<Metadata>>(&text).ok(),
        "application/json" => {
            serde_json::from_str::<cargo_manifest::Manifest<Metadata>>(&text).ok()
        }
        "application/yaml" => serde_yml::from_str::<cargo_manifest::Manifest<Metadata>>(&text).ok(),
        _ => return Ok(HttpResponse::UnsupportedMediaType().finish()),
    }) else {
        return Ok(HttpResponse::BadRequest().body("Invalid manifest"));
    };

    let Some(package) = manifest.package else {
        return Ok(HttpResponse::BadRequest().body("Magic keyword not provided"));
    };

    if !package
        .keywords
        .and_then(|k| k.as_local())
        .map(|k| k.contains(&"Christmas 2024".to_string()))
        .unwrap_or_default()
    {
        return Ok(HttpResponse::BadRequest().body("Magic keyword not provided"));
    }

    let Some(orders) = package.metadata.map(|m| {
        m.orders
            .into_iter()
            .filter(|o| o.quantity.is_some())
            .map(|o| format!("{}: {}", o.item, o.quantity.unwrap()))
            .collect::<Vec<_>>()
    }) else {
        return Ok(HttpResponse::NoContent().finish());
    };

    if orders.is_empty() {
        return Ok(HttpResponse::NoContent().finish());
    }

    Ok(HttpResponse::Ok().body(orders.join("\n")))
}
