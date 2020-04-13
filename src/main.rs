use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod routes {
    pub mod greet;
}

use routes::greet::{greet};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}