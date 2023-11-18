use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use data::PostcodeInfo;
use serde::{Deserialize, Serialize};
use simsearch::SimSearch;
mod data;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let postcode_info = data::postcode_info_from_file("data/zipcodes.de.json").expect(
        "Could not read postcode data from file."
    );

    let postcode_engine = data::build_engine(&postcode_info);

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .app_data(postcode_engine.clone())
            .service(zipcode_search)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
