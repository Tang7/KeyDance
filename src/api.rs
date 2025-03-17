use crate::acrcloud_api::AcrCloudClient;
use crate::models::AudioData;
use crate::staff_notation::StaffNotationGenerator;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::{error, info};

#[post("/api/recognize")]
pub async fn recognize_audio(audio_data: web::Json<AudioData>) -> impl Responder {
    info!("Received audio recognition request");

    let client = match AcrCloudClient::from_env() {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to initialize ACRCloud client: {}", e);
            return HttpResponse::InternalServerError().body(e);
        }
    };

    let recognition_result = client.recognize_base64_audio(&audio_data.data).await;
    client.create_recognition_response(recognition_result)
}

#[get("/api/staff/{song_id}")]
pub async fn get_staff_notation(path: web::Path<String>) -> impl Responder {
    let song_id = path.into_inner();
    info!("Received staff notation request for song ID: {}", song_id);

    let generator = StaffNotationGenerator::new();
    info!("Created StaffNotationGenerator");

    let notation = generator.generate_for_song(&song_id);
    info!("Generated staff notation: {:?}", notation);

    // Add CORS headers to ensure the response can be processed by the browser
    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .append_header(("Access-Control-Allow-Methods", "GET, POST, OPTIONS"))
        .append_header(("Access-Control-Allow-Headers", "Content-Type"))
        .json(notation)
}
