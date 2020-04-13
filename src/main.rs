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

    let port = get_port();

    let addr = match env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_e) => format!("0.0.0.0:{}", get_port()),
    };
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .wrap(Logger::default())
    })
    .bind(&addr)?
    .shutdown_timeout(1)
    .run()
    .await
}