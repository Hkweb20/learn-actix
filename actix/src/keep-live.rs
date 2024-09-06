use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::time::Duration;

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong")
}

async fn info() -> impl Responder {
    HttpResponse::Ok().body("Server Info: Actix Web with Keep-Alive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .route("/info", web::get().to(info))
    })
    .keep_alive(Duration::from_secs(60)) 
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
