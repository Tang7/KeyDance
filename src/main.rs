use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

mod api;
mod audio;
mod recognition;
mod staff_notation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let recognition_service = recognition::RecognitionService::new();

    println!("Starting music recognition server on http://127.0.0.1:8080");

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(recognition_service.clone()))
            .service(api::recognize_audio)
            .service(api::get_staff_notation)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?;

    println!("Server configured, starting...");

    server.run().await
}
