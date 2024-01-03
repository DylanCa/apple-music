use crate::controllers::script_controller::{ParamType, ScriptController};
use crate::models::application_data::ApplicationData;
use crate::models::error::Error;
use crate::models::playlist::Playlist;
use crate::models::track::Track;
use std::process::Output;
use strum_macros::Display;

pub struct AppleMusic;

impl AppleMusic {
    pub fn get_application_data() -> Result<ApplicationData, Error> {
        match ScriptController.execute_script::<ApplicationData>(
            ParamType::ApplicationData,
            None,
            None,
        ) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    pub fn get_playlist_by_id(id: i32) -> Result<Playlist, Error> {
        match ScriptController.execute_script::<Playlist>(ParamType::PlaylistById, Some(id), None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    pub fn get_current_track() -> Result<Track, Error> {
        match ScriptController.execute_script::<Track>(ParamType::CurrentTrack, None, None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    pub fn get_all_library_tracks() -> Result<Vec<Track>, Error> {
        match ScriptController.execute_script::<Vec<Track>>(ParamType::AllTracks, None, None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    pub fn play_track(track: &Track) -> Result<Output, Error> {
        let cmd = format!(
            "Application('Music').play(Application('Music').tracks.byId({}))",
            track.id
        );

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn play_playlist(playlist: &Playlist) -> Result<Output, Error> {
        let cmd = format!(
            "Application('Music').play(Application('Music').playlists.byId({}))",
            playlist.id
        );

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn set_mute(value: bool) -> Result<Output, Error> {
        let cmd = format!("Application('Music').mute = {}", value);

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn set_shuffle(value: bool) -> Result<Output, Error> {
        let cmd = format!("Application('Music').shuffleEnabled = {}", value);

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn set_song_repeat_mode(value: SongRepeatMode) -> Result<Output, Error> {
        let cmd = format!(
            "Application('Music').songRepeat = \"{}\"",
            value.to_string()
        );

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn convert_track(track: &Track) -> Result<Output, Error> {
        let cmd = format!(
            "Application('Music').convert(Application('Music').tracks.byId({}))",
            track.id
        );

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn play() -> Result<Output, Error> {
        let cmd = "Application('Music').play()";

        ScriptController.execute(cmd, None)
    }

    pub fn pause() -> Result<Output, Error> {
        let cmd = "Application('Music').pause()";

        ScriptController.execute(cmd, None)
    }

    pub fn resume() -> Result<Output, Error> {
        let cmd = "Application('Music').resume()";

        ScriptController.execute(cmd, None)
    }

    pub fn back_track() -> Result<Output, Error> {
        let cmd = "Application('Music').backTrack()";

        ScriptController.execute(cmd, None)
    }

    pub fn fast_forward() -> Result<Output, Error> {
        let cmd = "Application('Music').fastForward()";

        ScriptController.execute(cmd, None)
    }

    pub fn next_track() -> Result<Output, Error> {
        let cmd = "Application('Music').nextTrack()";

        ScriptController.execute(cmd, None)
    }

    pub fn playpause() -> Result<Output, Error> {
        let cmd = "Application('Music').playpause()";

        ScriptController.execute(cmd, None)
    }

    pub fn previous_track() -> Result<Output, Error> {
        let cmd = "Application('Music').previousTrack()";

        ScriptController.execute(cmd, None)
    }

    pub fn rewind() -> Result<Output, Error> {
        let cmd = "Application('Music').rewind()";

        ScriptController.execute(cmd, None)
    }

    pub fn stop() -> Result<Output, Error> {
        let cmd = "Application('Music').stop()";

        ScriptController.execute(cmd, None)
    }

    pub fn quit() -> Result<Output, Error> {
        let cmd = "Application('Music').quit()";

        ScriptController.execute(cmd, None)
    }

    pub fn run() -> Result<Output, Error> {
        let cmd = "Application('Music').run()";

        ScriptController.execute(cmd, None)
    }
}

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum SongRepeatMode {
    OFF,
    ONE,
    ALL,
}
