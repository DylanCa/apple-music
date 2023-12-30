use std::fmt::{Display, format};
use std::process::Command;
use log::debug;
use serde::Deserialize;
use crate::controllers::music::AppCommands;
use crate::models::error::Error;

pub struct ScriptController;

impl ScriptController {
    pub fn execute_script<T>(command: &str) -> Result<T, Error>
    where T: for<'a> Deserialize<'a> {
        let mut output = Command::new("osascript")
            .arg("-l")
            .arg("JavaScript")
            .arg("-e")
            .arg(command)
            .output();

        debug!("{:?}", output);

        let data;
        match output {
            Ok(d) => data = d,
            Err(err) => {
                debug!("{:?}", err);
                return Err(Error::NoData);
            }
        }

        let output_str = String::from_utf8_lossy(&data.stdout);

        return match serde_json::from_str::<T>(&output_str) {
            Ok(data) => Ok(data),
            Err(err) => {
                debug!("{:?}", err);
                Err(Error::DeserializationFailed)
            }
        }
    }

    pub fn execute(command: AppCommands) {
        let cmd = format!("Application('Music').{}();", command.to_string());

        let output = Command::new("osascript")
            .arg("-l")
            .arg("JavaScript")
            .arg("-e")
            .arg(cmd)
            .output();

        debug!("{:?}", output);
    }
}