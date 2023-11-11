use actix_web::{App, HttpServer};

mod drawing;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(drawing::drawing))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
