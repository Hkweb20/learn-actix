use std::{collections::HashMap, sync::Mutex};
use actix_web::{get, guard, post, web::{self, to}, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use serde::{Serialize};
use rand::Rng;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}