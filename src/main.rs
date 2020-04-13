use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::env;

mod routes {
    pub mod greet;
}

use routes::greet::{greet};

fn get_port() -> u16 {
    if let Ok(port_var) = env::var("PORT") {
        if let Ok(port) = port_var.parse::<u16>() {
            return port;
        }
    }
    8080
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(format!("127.0.0.1:{}", get_port()))?
    .run()
    .await
}