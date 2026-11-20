use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

mod lib;
use lib::roster::{get_lineup};

#[derive(Deserialize)]
struct QueryParams {
    team: String,
    starters_only: bool,
}

#[get("/")]
async fn index(query: web::Query<QueryParams>) -> impl Responder {
    get_lineup(query.team, query.starters_only)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}