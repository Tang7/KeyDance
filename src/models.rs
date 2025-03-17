use serde::{Deserialize, Serialize};

// Request models
#[derive(Deserialize)]
pub struct AudioData {
    pub data: String, // Base64 encoded audio data
}

// Response models
#[derive(Debug, Serialize, Deserialize)]
pub struct MusicRecognitionResult {
    pub title: String,
    pub artist: String,
    pub confidence: f32,
    pub song_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffNotation {
    pub title: String,
    pub notation: String,
}
