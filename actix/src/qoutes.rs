use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize};
use rand::Rng;

#[derive(Debug, Serialize)]  // Derive Serialize for Quotes struct
struct Quotes {
    title : String,
    author : String
}
struct Appstate {
    qoutes: HashMap<&'static str, Quotes>,
}

#[get("/qoutes")]
async fn qoutes (data: web::Data<Appstate>) -> impl Responder{
    let qoutes = &data.qoutes;

    let mut rng = rand::thread_rng();
    let random_qoutes = rng.gen_range(1..=qoutes.len()).to_string();
   
     if let Some(random_q) = qoutes.get(random_qoutes.as_str())  {
         HttpResponse::Ok().json(random_q)
     }else {
         HttpResponse::NotFound().body("Qoutes Not Found")
     }
   }
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let mut qoutes_values :HashMap<&str, Quotes>=HashMap::new();
    qoutes_values.insert("1", Quotes { title: "Tada hankali".to_string(), author:"Hassan Kabiru".to_string() });
    qoutes_values.insert("2", Quotes { title: "Bari Mugani".to_string(), author: "Hussaini Kabiru".to_string() });

    let appdata = web::Data::new( Appstate {qoutes: qoutes_values});

    HttpServer::new(move|| {
        App::new().app_data(appdata.clone()).service(qoutes)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}