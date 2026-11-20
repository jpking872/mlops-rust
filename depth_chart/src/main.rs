use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod mylib;
use mylib::roster::get_lineup;

#[derive(Deserialize)]
struct QueryParams {
    team: String
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json({"Welcome".to_string()})
}

#[get("/players")]
async fn get_players(query: web::Query<QueryParams>) -> impl Responder {
    let players = get_lineup(query.team.to_string()).await.unwrap();
    HttpResponse::Ok().json(players)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8081");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_players)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}