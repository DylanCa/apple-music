use strum_macros::Display;
use crate::controllers::script_controller::ScriptController;
use crate::models::application_data::ApplicationData;
use crate::models::track::Track;

pub struct AppleMusic;

impl AppleMusic {
    pub fn get_application_data() -> ApplicationData {
        ScriptController::execute_script::<ApplicationData>(include_str!("./scripts/application.js")).unwrap()
    }

    pub fn get_current_track() -> Track {
        ScriptController::execute_script::<Track>(include_str!("./scripts/current_track.js")).unwrap()
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
