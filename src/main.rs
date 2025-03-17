use actix_cors::Cors;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;

mod acrcloud_api;
mod api;
mod models;
mod staff_notation;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting Key Dance application");

    info!("Starting music recognition server on http://127.0.0.1:8080");

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        info!("Configuring application routes");
        App::new()
            .wrap(cors)
            .service(api::recognize_audio)
            .service(api::get_staff_notation)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?;

    info!("Server configured, starting...");

    server.run().await
}
