use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web::middleware::Logger;
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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .wrap(Logger::default())
    })
    .bind(format!("127.0.0.1:{}", get_port()))?
    .run()
    .await
}