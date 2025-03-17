use crate::recognition::RecognitionService;
use actix_web::{get, post, web, HttpResponse, Responder};
use base64::engine::Engine;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AudioData {
    data: String, // Base64 encoded audio data
}

#[post("/api/recognize")]
pub async fn recognize_audio(
    audio_data: web::Json<AudioData>,
    recognition_service: web::Data<RecognitionService>,
) -> impl Responder {
    // Decode the base64 audio data
    let decoded: Vec<u8> = match base64::engine::general_purpose::STANDARD.decode(&audio_data.data)
    {
        Ok(data) => data,
        Err(_) => return HttpResponse::BadRequest().body("Invalid audio data"),
    };

    // Process the audio data with the recognition service
    match recognition_service.recognize_audio(&decoded).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[get("/api/staff/{song_id}")]
pub async fn get_staff_notation(
    path: web::Path<String>,
    recognition_service: web::Data<RecognitionService>,
) -> impl Responder {
    let song_id = path.into_inner();

    match recognition_service.get_staff_notation(&song_id).await {
        Ok(notation) => HttpResponse::Ok().json(notation),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
