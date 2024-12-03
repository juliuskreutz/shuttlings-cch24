use std::net::{Ipv4Addr, Ipv6Addr};

use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};

use crate::ShuttleResult;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(get_dest)
        .service(get_key)
        .service(get_v6_dest)
        .service(get_v6_key);
}

#[derive(serde::Deserialize)]
struct DestInfo {
    from: String,
    key: String,
}

#[derive(serde::Deserialize)]
struct KeyInfo {
    from: String,
    to: String,
}

#[get("/2/dest")]
async fn get_dest(info: web::Query<DestInfo>) -> ShuttleResult<impl Responder> {
    let from: Ipv4Addr = info.from.parse()?;
    let key: Ipv4Addr = info.key.parse()?;

    let mut octets = [0; 4];

    for (i, o) in octets.iter_mut().enumerate() {
        *o = from.octets()[i].overflowing_add(key.octets()[i]).0;
    }

    let dest = Ipv4Addr::from(octets);

    Ok(dest.to_string())
}

#[get("/2/key")]
async fn get_key(info: web::Query<KeyInfo>) -> ShuttleResult<impl Responder> {
    let from: Ipv4Addr = info.from.parse()?;
    let to: Ipv4Addr = info.to.parse()?;

    let mut octets = [0; 4];

    for (i, o) in octets.iter_mut().enumerate() {
        *o = to.octets()[i].overflowing_sub(from.octets()[i]).0;
    }

    let key = Ipv4Addr::from(octets);

    Ok(key.to_string())
}

#[get("/2/v6/dest")]
async fn get_v6_dest(info: web::Query<DestInfo>) -> ShuttleResult<impl Responder> {
    let from: Ipv6Addr = info.from.parse()?;
    let key: Ipv6Addr = info.key.parse()?;

    let mut segments = [0; 8];

    for (i, o) in segments.iter_mut().enumerate() {
        *o = from.segments()[i] ^ key.segments()[i];
    }

    let dest = Ipv6Addr::from(segments);

    Ok(dest.to_string())
}

#[get("/2/v6/key")]
async fn get_v6_key(info: web::Query<KeyInfo>) -> ShuttleResult<impl Responder> {
    let from: Ipv6Addr = info.from.parse()?;
    let to: Ipv6Addr = info.to.parse()?;

    let mut segments = [0; 8];

    for (i, o) in segments.iter_mut().enumerate() {
        *o = to.segments()[i] ^ from.segments()[i];
    }

    let dest = Ipv6Addr::from(segments);

    Ok(dest.to_string())
}
