//! File: ./to_do/networking/actix_server/src/main.rs
mod api;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};

// File: ./to_do/networking/actix_server/src/main.rs
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

// File: ./to_do/networking/actix_server/src/main.rs
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(api::views_factory)
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}