use serde_json::json;
use strum_macros::Display;
use crate::controllers::script_controller::{ParamType, ScriptController};
use crate::models::application_data::ApplicationData;
use crate::models::error::Error;
use crate::models::track::Track;

pub struct AppleMusic;

impl AppleMusic {
    pub fn get_application_data() -> Result<ApplicationData, Error> {
        match ScriptController::execute_script::<ApplicationData>(include_str!("./scripts/application.js"), None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err)
        }
    }

    pub fn get_current_track() -> Result<Track, Error> {
        let params = json!({"param": ParamType::CurrentTrack.to_string()}).to_string();

        match ScriptController::execute_script::<Track>(include_str!("./scripts/tracks.js"), Some(params)) {
            Ok(data) => Ok(data),
            Err(err) => Err(err)
        }
    }

    pub fn get_all_library_tracks() -> Result<Vec<Track>, Error> {
        let params = json!({"param": ParamType::AllTracks.to_string()}).to_string();

        match ScriptController::execute_script::<Vec<Track>>(include_str!("./scripts/tracks.js"), Some(params)) {
            Ok(data) => Ok(data),
            Err(err) => Err(err)
        }
    }

    pub fn execute(command: AppCommands) {
        ScriptController::execute(command);
    }
}

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum AppCommands {
    QUIT, // Quit the application
    RUN, // Run the application
    PLAY, // play the current track or the specified track or file.
    PAUSE, // pause playback
    RESUME, // disable fast forward/rewind and resume playback, if playing.

    #[strum(serialize = "backTrack")]
    BACKTRACK, // reposition to beginning of current track or go to previous track if already at start of current track

    #[strum(serialize = "fastForward")]
    FASTFORWARD, // skip forward in a playing track

    #[strum(serialize = "nextTrack")]
    NEXTTRACK, // advance to the next track in the current playlist
    PLAYPAUSE,

    #[strum(serialize = "previousTrack")]
    PREVIOUSTRACK,
    REWIND, // skip backwards in a playing track
    STOP, // stop playback

    /* TODO:
    STANDARD SUITE +
    PLAY SPECIFIER
    ADD,
    CONVERT,
    DOWNLOAD,
    EXPORT,
    REFRESH,
    REVEAL,
    SEARCH,
    SELECT,

    */
}
