use std::net::{Ipv4Addr, Ipv6Addr};

use actix_web::{get, web};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_dest)
        .service(get_key)
        .service(get_v6_dest)
        .service(get_v6_key);
}

#[derive(serde::Deserialize)]
struct DestInfo {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

#[get("/2/dest")]
async fn get_dest(web::Query(DestInfo { from, key }): web::Query<DestInfo>) -> String {
    let mut octets = [0; 4];

    for (i, o) in octets.iter_mut().enumerate() {
        *o = from.octets()[i].wrapping_add(key.octets()[i]);
    }

    Ipv4Addr::from(octets).to_string()
}

#[derive(serde::Deserialize)]
struct KeyInfo {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

#[get("/2/key")]
async fn get_key(web::Query(KeyInfo { from, to }): web::Query<KeyInfo>) -> String {
    let mut octets = [0; 4];

    for (i, o) in octets.iter_mut().enumerate() {
        *o = to.octets()[i].wrapping_sub(from.octets()[i]);
    }

    Ipv4Addr::from(octets).to_string()
}

#[derive(serde::Deserialize)]
struct DestInfoV6 {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

#[get("/2/v6/dest")]
async fn get_v6_dest(web::Query(DestInfoV6 { from, key }): web::Query<DestInfoV6>) -> String {
    let mut segments = [0; 8];

    for (i, o) in segments.iter_mut().enumerate() {
        *o = from.segments()[i] ^ key.segments()[i];
    }

    Ipv6Addr::from(segments).to_string()
}

#[derive(serde::Deserialize)]
struct KeyInfoV6 {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

#[get("/2/v6/key")]
async fn get_v6_key(web::Query(KeyInfoV6 { from, to }): web::Query<KeyInfoV6>) -> String {
    let mut segments = [0; 8];

    for (i, o) in segments.iter_mut().enumerate() {
        *o = to.segments()[i] ^ from.segments()[i];
    }

    Ipv6Addr::from(segments).to_string()
}
