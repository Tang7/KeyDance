use crate::models::StaffNotation;

pub struct StaffNotationGenerator;

impl StaffNotationGenerator {
    pub fn new() -> Self {
        StaffNotationGenerator
    }

    pub fn generate(&self) -> StaffNotation {
        // TODO: return a placeholder notation for now
        StaffNotation {
            title: "Example Staff Notation".to_string(),
            notation: "C4 D4 E4 F4 | G4 A4 B4 C5 | C5 B4 A4 G4 | F4 E4 D4 C4".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn generate_for_song(&self, song_id: &str) -> StaffNotation {
        log::info!("Generating staff notation for song ID: {}", song_id);

        self.generate()
    }
}
