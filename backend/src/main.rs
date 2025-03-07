use actix_web::{App, HttpServer};

mod routes;
mod controllers;
mod utils;
mod models;

// ZKP Generator and Verifier Server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting backend server on 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .configure(routes::age_verification::init_routes)
            .configure(routes::citizenship::init_routes)
            .configure(routes::college_degree::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
