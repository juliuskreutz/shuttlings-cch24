use std::collections::HashSet;

use actix_web::{http, post, web, HttpRequest, HttpResponse, Responder};

use crate::ShuttleResult;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(post_manifest);
}

#[derive(serde::Deserialize)]
struct Toml {
    package: Package,
}

#[derive(serde::Deserialize)]
struct Package {
    #[serde(default)]
    metadata: Metadata,
    #[serde(default)]
    keywords: HashSet<String>,
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

    let content_type = content_type.to_str()?;

    if !matches!(
        content_type,
        "application/toml" | "application/json" | "application/yaml"
    ) {
        return Ok(HttpResponse::UnsupportedMediaType().finish());
    }

    if match content_type {
        "application/toml" => toml::from_str::<cargo_manifest::Manifest>(&text).is_err(),
        "application/json" => serde_json::from_str::<cargo_manifest::Manifest>(&text).is_err(),
        "application/yaml" => serde_yml::from_str::<cargo_manifest::Manifest>(&text).is_err(),
        _ => unreachable!(),
    } {
        return Ok(HttpResponse::BadRequest().body("Invalid manifest"));
    }

    let toml: Toml = match content_type {
        "application/toml" => toml::from_str(&text)?,
        "application/json" => serde_json::from_str(&text)?,
        "application/yaml" => serde_yml::from_str(&text)?,
        _ => unreachable!(),
    };

    if !toml.package.keywords.contains("Christmas 2024") {
        return Ok(HttpResponse::BadRequest().body("Magic keyword not provided"));
    }

    let body = toml
        .package
        .metadata
        .orders
        .into_iter()
        .filter(|o| o.quantity.is_some())
        .map(|o| format!("{}: {}", o.item, o.quantity.unwrap()))
        .collect::<Vec<_>>()
        .join("\n");

    if body.is_empty() {
        return Ok(HttpResponse::NoContent().finish());
    }

    Ok(HttpResponse::Ok().body(body))
}
