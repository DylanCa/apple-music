use std::fmt::{Display, format};
use std::process::Command;
use log::debug;
use crate::models::error::Error;
use crate::models::track::Track;

pub struct AppleMusic;

impl AppleMusic {
    pub fn get_current_track() -> Result<Track, Error> {
        let mut output = Command::new("osascript")
            .arg("-l")
            .arg("JavaScript")
            .arg("-e")
            .arg(include_str!("../scripts/current_track.js"))
            .output();

        let data;
        match output {
            Ok(d) => data = d,
            Err(err) => {
                debug!("{:?}", err);
                return Err(Error::NoData);
            }
        }

        let output_str = String::from_utf8_lossy(&data.stdout);

        return match serde_json::from_str::<Track>(&output_str) {
            Ok(data) => Ok(data),
            Err(err) => {
                debug!("{:?}", err);
                Err(Error::DeserializationFailed)
            }
        }
    }
}
