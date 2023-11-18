use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let postcode_info = data::postcode_info_from_file("data/zipcodes.de.json").expect(
        "Could not read postcode data from file."
    );

    for info in postcode_info {
        println!("{:?}", info);
    }



    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
