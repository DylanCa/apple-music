use std::fmt::format;
use std::process::Command;
use crate::models::track::Track;

pub struct AppleMusic;

impl AppleMusic {
    pub fn current_track() -> Result<Track, String> {
        let output = Command::new("osascript")
            .arg("-l")
            .arg("JavaScript")
            .arg("-e")
            .arg(include_str!("../scripts/current_track.js"))
            .output()
            .expect("Failed to get current_track Data!");

        let output_str = String::from_utf8_lossy(&output.stdout);

        match serde_json::from_str::<Track>(&output_str) {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("Failed to deserialize current_track Data! : {}", err.to_string()))
        }
    }
}
