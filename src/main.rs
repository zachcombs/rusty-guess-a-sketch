use actix_cors::Cors;
use actix_web::{http, App, HttpServer};

mod drawing;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(drawing::drawing)
            .service(drawing::drawing_from_category)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
