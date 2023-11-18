use std::collections::HashMap;

use actix_web::{
    get, post,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use data::PostcodeInfo;
use map::Map;
use serde::{Deserialize, Serialize};
use simsearch::SimSearch;

use crate::data::ServiceProvider;
mod data;
mod map;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize, Deserialize)]
struct searchRequest {
    q: String,
}
#[get("/zipcode/search")]
async fn zipcode_search(req: HttpRequest, query: web::Query<searchRequest>) -> impl Responder {
    let postcode_engine: &SimSearch<PostcodeInfo> = req
        .app_data()
        .expect("Postcode engine not found in app data.");

    let mut res = postcode_engine.search(&query.q);
    res.truncate(10);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res).unwrap())
}

#[get("/craftsmen/{postalcode}")]
async fn craftsmen_search(req: HttpRequest, path: web::Path<String>) -> Result<impl Responder> {
    let postalcode = path.into_inner();
    let map: &Map = req.app_data().expect("Map not found!");

    Ok(HttpResponse::Ok().content_type("application/json").body(
        if let Some(service_providers) = map.ranked_by_score(postalcode.parse().unwrap()) {
            serde_json::to_string(&service_providers).unwrap()
        } else {
            "[]".to_string()
        },
    ))
}

#[derive(Serialize, Deserialize)]
struct detailedRequest {
    page: Option<u32>,
    sort: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct detailedResponse {
    has_more: bool,
    total_count: usize,
    results: Vec<ServiceProvider>,
}

#[get("/craftsmen/{postalcode}/detailed")]
async fn craftsmen_search_detailed(
    req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<detailedRequest>,
) -> Result<impl Responder> {
    let postalcode = path.into_inner();
    let map: &Map = req.app_data().expect("Map not found!");

    let Some(mut service_providers) = (match query.sort.as_deref() {
        Some("distance") => map.ranked_by_distance(postalcode.parse().unwrap()),
        Some("profile") => map.ranked_by_profile(postalcode.parse().unwrap()),
        _ => map.ranked_by_score(postalcode.parse().unwrap()),
    }) else {
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body("[]".to_string()));
    };

    let total_count = service_providers.len();

    const PAGE_SIZE: u32 = 20;
    let start = query.page.unwrap_or(0) * PAGE_SIZE;

    service_providers = service_providers.split_off(start as usize);
    let has_more = service_providers.len() > PAGE_SIZE as usize;
    service_providers.truncate(PAGE_SIZE as usize);

    let detailed: Vec<ServiceProvider> = service_providers
        .iter()
        .map(|sp| map.service_provider_by_id(sp.id))
        .filter(|sp| sp.is_some())
        .map(|sp| sp.unwrap())
        .collect();

    Ok(HttpResponse::Ok().content_type("application/json").body(
        serde_json::to_string(&detailedResponse {
            has_more,
            total_count: total_count,
            results: detailed,
        })
        .unwrap(),
    ))
}

pub fn build_engine(postcodes: &Vec<PostcodeInfo>) -> SimSearch<PostcodeInfo> {
    let mut engine: SimSearch<PostcodeInfo> = SimSearch::new();

    for info in postcodes {
        let search_str = info.zipcode.to_string() + " " + &info.place;
        engine.insert(info.clone(), &search_str);
    }

    engine
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let postcode_info = data::postcode_info_from_file("data/zipcodes.de.json")
        .expect("Could not read postcode data from file.");

    let postcodes = data::postcode_from_file().unwrap();
    let service_providers = data::provider_from_file().unwrap();
    let quality_factor = data::quality_from_file().unwrap();

    let map = Map::new(postcodes, quality_factor, service_providers);

    let postcode_engine = build_engine(&postcode_info);

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .app_data(postcode_engine.clone())
            .app_data(map.clone())
            .service(zipcode_search)
            .service(craftsmen_search)
            .service(craftsmen_search_detailed)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
