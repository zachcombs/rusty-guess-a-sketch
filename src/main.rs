use actix_cors::Cors;
use actix_web::{http, App, HttpServer};

mod drawing;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            //SWAP NEXT TWO LINES FOR LOCAL vs REMOTE
            // .allowed_origin("http://localhost:5173")
            .allowed_origin("https://guess-a-sketch-6hes.onrender.com")
            .allowed_origin("https://guess-a-sketch-git-develop-zachcombs-projects.vercel.app")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://guess-a-sketch")
                    && origin.as_bytes().ends_with(b".vercel.app")
            })
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(drawing::drawing)
            .service(drawing::drawing_from_category)
    })
    //SWAP NEXT TWO LINES FOR LOCAL vs REMOTE
    //.bind("127.0.0.1", 8080)
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
