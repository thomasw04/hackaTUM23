use std::sync::RwLock;

use actix_web::patch;
use actix_web::web::Data;
use actix_web::{
    get,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

use data::PostcodeInfo;
use env_logger::Env;
use map::Map;
use serde::{Deserialize, Serialize};
use simsearch::SimSearch;

use crate::data::ServiceProvider;
mod data;
mod map;

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    q: String,
}

#[get("/zipcode/search")]
async fn zipcode_search(req: HttpRequest, query: web::Query<SearchRequest>) -> impl Responder {
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
async fn craftsmen_search(
    path: web::Path<String>,
    data: Data<RwLock<Map>>,
) -> Result<impl Responder> {
    let postalcode = path.into_inner();
    let map = data.read().unwrap();

    Ok(HttpResponse::Ok().content_type("application/json").body(
        if let Some(service_providers) = map.ranked_by_score(postalcode.parse().unwrap()) {
            serde_json::to_string(&service_providers).unwrap()
        } else {
            "[]".to_string()
        },
    ))
}

#[derive(Deserialize)]
struct UpdateRequest {
    maxDrivingDistance: Option<u64>,
    profilePictureScore: Option<f64>,
    profileDescriptionScore: Option<f64>,
}

#[derive(Serialize)]
struct UpdateResponse {
    id: u32,
    updated: UpdatedFields,
}

#[derive(Serialize)]
struct UpdatedFields {
    maxDrivingDistance: u64,
    profilePictureScore: f64,
    profileDescriptionScore: f64,
}

#[patch("/craftman/{craftman_id}")]
async fn craftsmen_update(
    info: web::Json<UpdateRequest>,
    path: web::Path<String>,
    data: Data<RwLock<Map>>,
) -> Result<impl Responder> {
    let craftmen_id = path.into_inner().parse().unwrap();
    let mut map = data.write().unwrap();

    let (maxDrivingDistance, profilePictureScore, profileDescriptionScore) = map
        .update_service_provider(
            craftmen_id,
            info.maxDrivingDistance,
            info.profilePictureScore,
            info.profileDescriptionScore,
        );

    let updated_fields = UpdatedFields {
        maxDrivingDistance,
        profilePictureScore,
        profileDescriptionScore,
    };

    let response = UpdateResponse {
        id: craftmen_id,
        updated: updated_fields,
    };

    // Return the PatchResponse in the HTTP response
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Serialize, Deserialize)]
struct DetailedRequest {
    page: Option<u32>,
    sort: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DetailedResponse {
    has_more: bool,
    total_count: usize,
    results: Vec<ServiceProvider>,
}

#[get("/craftsmen/{postalcode}/detailed")]
async fn craftsmen_search_detailed(
    path: web::Path<String>,
    query: web::Query<DetailedRequest>,
    data: Data<RwLock<Map>>,
) -> Result<impl Responder> {
    let postalcode = path.into_inner();
    let map = data.read().unwrap();

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
        serde_json::to_string(&DetailedResponse {
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
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let postcode_info = data::postcode_info_from_file("data/zipcodes.de.json")
        .expect("Could not read postcode data from file.");

    let postcodes = data::postcode_from_file().unwrap();
    let service_providers = data::provider_from_file().unwrap();
    let quality_factor = data::quality_from_file().unwrap();

    let map = Data::new(RwLock::new(Map::new(
        postcodes,
        quality_factor,
        service_providers,
    )));

    let postcode_engine = build_engine(&postcode_info);

    HttpServer::new(move || {
        App::new()
            .app_data(postcode_engine.clone())
            .app_data(Data::clone(&map))
            .service(zipcode_search)
            .service(craftsmen_search)
            .service(craftsmen_search_detailed)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
