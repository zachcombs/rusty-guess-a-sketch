use rand::{seq::IteratorRandom, Rng};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::io;

mod drawing;



// #[tokio::main]
#[actix_web::main]
async fn main() -> io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8081")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(drawing::drawing)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
