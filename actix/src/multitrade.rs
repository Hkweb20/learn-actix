use actix_web::{web, App, HttpServer, Responder};
use std::sync::{Arc, Mutex};

struct AppState {
    counter: Arc<Mutex<i32>>,
}

async fn increment(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Counter incremented: {}", counter)
}

async fn get_value(data: web::Data<AppState>) -> impl Responder {
    let counter = data.counter.lock().unwrap();
    format!("Counter value: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(Mutex::new(0));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                counter: counter.clone(),
            }))
            .route("/increment", web::get().to(increment))
            .route("/value", web::get().to(get_value))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
