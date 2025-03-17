use crate::models::MusicRecognitionResult;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::Deserialize;
use sha1::Sha1;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct AcrCloudResponse {
    pub status: Status,
    #[serde(default)]
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub code: i32,
    pub msg: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Metadata {
    #[serde(default)]
    pub music: Vec<Music>,
}

#[derive(Debug, Deserialize)]
pub struct Music {
    pub title: String,
    pub artists: Vec<Artist>,
    #[serde(default)]
    pub score: f64,
    pub acrid: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub name: String,
}

pub struct AcrCloudClient {
    host: String,
    access_key: String,
    access_secret: String,
    client: Client,
}

impl AcrCloudClient {
    pub fn new(host: &str, access_key: &str, access_secret: &str) -> Self {
        Self {
            host: host.to_string(),
            access_key: access_key.to_string(),
            access_secret: access_secret.to_string(),
            client: Client::new(),
        }
    }

    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();

        let host = env::var("ACRCLOUD_HOST")
            .map_err(|_| "ACRCLOUD_HOST must be set in .env file".to_string())?;

        let access_key = env::var("ACRCLOUD_ACCESS_KEY")
            .map_err(|_| "ACRCLOUD_ACCESS_KEY must be set in .env file".to_string())?;

        let access_secret = env::var("ACRCLOUD_ACCESS_SECRET")
            .map_err(|_| "ACRCLOUD_ACCESS_SECRET must be set in .env file".to_string())?;

        Ok(Self::new(&host, &access_key, &access_secret))
    }

    fn create_signature(
        &self,
        timestamp: &str,
        data_type: &str,
        signature_version: &str,
    ) -> String {
        let string_to_sign = format!(
            "POST\n/v1/identify\n{}\n{}\n{}\n{}",
            self.access_key, data_type, signature_version, timestamp
        );

        let mut mac = Hmac::<Sha1>::new_from_slice(self.access_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        BASE64.encode(mac.finalize().into_bytes())
    }

    #[allow(dead_code)]
    pub async fn identify_file(&self, file_path: &Path) -> Result<AcrCloudResponse, String> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to open audio file: {}", e)),
        };

        let mut audio_bytes = Vec::new();
        if let Err(e) = file.read_to_end(&mut audio_bytes) {
            return Err(format!("Failed to read audio file: {}", e));
        }

        self.identify_audio(
            &audio_bytes,
            file_path.file_name().unwrap().to_str().unwrap(),
        )
        .await
    }

    pub async fn identify_audio(
        &self,
        audio_bytes: &[u8],
        filename: &str,
    ) -> Result<AcrCloudResponse, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let data_type = "audio";
        let signature_version = "1";

        let form = self.create_identify_form(
            audio_bytes,
            filename,
            &timestamp,
            data_type,
            signature_version,
        );

        let url = format!("https://{}/v1/identify", self.host);

        match self.client.post(&url).multipart(form).send().await {
            Ok(response) => {
                if !response.status().is_success() {
                    return Err(format!("Request failed with status: {}", response.status()));
                }

                let text = match response.text().await {
                    Ok(text) => text,
                    Err(e) => return Err(format!("Failed to read response: {}", e)),
                };

                match serde_json::from_str::<AcrCloudResponse>(&text) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }

    // Helper method to decode base64 data
    fn decode_base64(&self, base64_data: &str) -> Result<Vec<u8>, String> {
        BASE64
            .decode(base64_data)
            .map_err(|e| format!("Failed to decode base64 audio data: {}", e))
    }

    // Helper method to process ACRCloud response
    fn process_response(
        &self,
        response: AcrCloudResponse,
    ) -> Result<MusicRecognitionResult, String> {
        if response.status.code != 0 {
            return Err(format!(
                "ACRCloud error: {} (code: {})",
                response.status.msg, response.status.code
            ));
        }

        if response.metadata.music.is_empty() {
            return Err("No music matches found".to_string());
        }

        // Get the first (best) match
        let music = &response.metadata.music[0];

        let artist = if !music.artists.is_empty() {
            music.artists[0].name.clone()
        } else {
            "Unknown".to_string()
        };

        // Extract the song_id from ACRCloud's response or generate one
        let song_id = match &music.acrid {
            Some(id) => id.clone(),
            None => {
                // If ACRCloud didn't provide an ID, generate one from title and artist
                format!(
                    "{}-{}",
                    music.title.to_lowercase().replace(' ', "-"),
                    artist.to_lowercase().replace(' ', "-")
                )
            }
        };

        let result = MusicRecognitionResult {
            title: music.title.clone(),
            artist,
            confidence: (music.score / 100.0) as f32,
            song_id,
        };

        Ok(result)
    }

    // Helper method to create multipart form
    fn create_identify_form(
        &self,
        audio_bytes: &[u8],
        filename: &str,
        timestamp: &str,
        data_type: &str,
        signature_version: &str,
    ) -> reqwest::multipart::Form {
        let signature = self.create_signature(timestamp, data_type, signature_version);

        reqwest::multipart::Form::new()
            .part(
                "sample",
                reqwest::multipart::Part::bytes(audio_bytes.to_vec())
                    .file_name(filename.to_string())
                    .mime_str("audio/wav")
                    .unwrap_or_else(|_| {
                        reqwest::multipart::Part::bytes(audio_bytes.to_vec())
                            .file_name(filename.to_string())
                    }),
            )
            .text("access_key", self.access_key.clone())
            .text("sample_bytes", audio_bytes.len().to_string())
            .text("timestamp", timestamp.to_string())
            .text("signature", signature)
            .text("data_type", data_type.to_string())
            .text("signature_version", signature_version.to_string())
    }

    pub async fn recognize_base64_audio(
        &self,
        base64_data: &str,
    ) -> Result<MusicRecognitionResult, String> {
        let decoded = self.decode_base64(base64_data)?;

        let response = self.identify_audio(&decoded, "recorded_audio.wav").await?;

        self.process_response(response)
    }

    pub fn create_recognition_response(
        &self,
        result: Result<MusicRecognitionResult, String>,
    ) -> actix_web::HttpResponse {
        match result {
            Ok(recognition_result) => {
                log::info!(
                    "Song identified: '{}' by '{}' with confidence {:.2}%",
                    recognition_result.title,
                    recognition_result.artist,
                    recognition_result.confidence * 100.0
                );
                actix_web::HttpResponse::Ok().json(recognition_result)
            }
            Err(error_message) => {
                log::error!("Recognition failed: {}", error_message);

                if error_message.contains("No music matches found") {
                    actix_web::HttpResponse::NotFound().body(error_message)
                } else if error_message.contains("Failed to decode base64") {
                    actix_web::HttpResponse::BadRequest().body(error_message)
                } else {
                    actix_web::HttpResponse::InternalServerError().body(error_message)
                }
            }
        }
    }
}
