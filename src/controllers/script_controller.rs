use std::borrow::Cow;
use std::process::{Command, Output};
use log::debug;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use crate::controllers::music::AppCommands;
use crate::models::error::Error;

#[derive(Serialize, Display)]
#[strum(serialize_all = "camelCase")]
pub enum ParamType {
    None,
    AllTracks,
    CurrentTrack,
    PlaylistTracks,
    SearchInPlaylist,
}

pub struct ScriptController;

impl ScriptController {
    pub fn execute_script<T>(&self, command: &str, params: Option<String>) -> Result<T, Error>
    where T: for<'a> Deserialize<'a> {
        let output = self.execute(command, params);
        // debug!("{:#?}", output_str);

        let data;
        match output {
            Ok(d) => data = d,
            Err(err) => {
                return Err(err);
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

    pub fn execute(&self, command: &str, params: Option<String>) -> Result<Output, Error> {
        let mut binding = Command::new("osascript");
        let output = binding.arg("-l")
            .arg("JavaScript")
            .arg("-e")
            .arg(command);

        if let Some(params) = params {
            output.arg(params);
        }
        let data;
        match output.output() {
            Ok(d) => data = d,
            Err(err) => {
                debug!("{:?}", err);
                return Err(Error::NoData)
            }
        }

        return Ok(data)
    }
}