use crate::application_data::ApplicationData;
use crate::error::Error;
use crate::playlist::Playlist;
use crate::script_controller::{ParamType, ScriptController};
use crate::track::Track;
use log::warn;
use strum_macros::Display;

/// Strict entry point of the module containing the whole logic.
pub struct AppleMusic;

impl AppleMusic {
    /// Returns an up-to-date ApplicationData struct.
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

    /// Looks for and returns a Playlist based on provided id, if it exists.
    pub fn get_playlist_by_id(id: i32) -> Result<Playlist, Error> {
        match ScriptController.execute_script::<Playlist>(ParamType::PlaylistById, Some(id), None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    /// Returns currently playing Track, if any.
    pub fn get_current_track() -> Result<Track, Error> {
        match ScriptController.execute_script::<Track>(ParamType::CurrentTrack, None, None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    /// Fetches and returns a list of all Library Tracks.
    /// WARNING: Might fail if more than 900 Tracks are to be returned, due to a JavaScript limit.
    pub fn get_all_library_tracks() -> Result<Vec<Track>, Error> {
        warn!("get_all_library_tracks() Might fail if more than 900 Tracks are to be returned, due to a JavaScript limit.");
        match ScriptController.execute_script::<Vec<Track>>(ParamType::AllTracks, None, None) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        }
    }

    /// Plays the provided Track on AppleMusic player.
    pub fn play_track(track: &Track) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').play(Application('Music').tracks.byId({}))",
            track.id
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Plays the provided Playlist on AppleMusic player.
    pub fn play_playlist(playlist: &Playlist) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').play(Application('Music').playlists.byId({}))",
            playlist.id
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Mutes / Unmutes AppleMusic player.
    pub fn set_mute(value: bool) -> Result<(), Error> {
        let cmd = format!("Application('Music').mute = {}", value);

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Activates / Deactivates Shuffle mode on AppleMusic player.
    pub fn set_shuffle(value: bool) -> Result<(), Error> {
        let cmd = format!("Application('Music').shuffleEnabled = {}", value);

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Sets Song Repeat mode to provided value.
    pub fn set_song_repeat_mode(value: SongRepeatMode) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').songRepeat = \"{}\"",
            value.to_string()
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Sets Sound Volume to provided value. ( 0 <= value <= 100 )
    pub fn set_sound_volume(value: i8) -> Result<(), Error> {
        let cmd = format!("Application('Music').soundVolume = {}", value);

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Tries to convert the provided Track.
    pub fn convert_track(track: &Track) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').convert(Application('Music').tracks.byId({}))",
            track.id
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Resumes the player if a track is Paused, otherwise Plays a Track from Library.
    pub fn play() -> Result<(), Error> {
        let cmd = "Application('Music').play()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Pauses the player's currently playing Track.
    pub fn pause() -> Result<(), Error> {
        let cmd = "Application('Music').pause()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Stops Rewinding / Fast-Forwarding and plays the Track at normal speed.
    pub fn resume() -> Result<(), Error> {
        let cmd = "Application('Music').resume()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Restart the current Track.
    pub fn back_track() -> Result<(), Error> {
        let cmd = "Application('Music').backTrack()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Fast-forwards the current Track up until resuming or end of current Track.
    pub fn fast_forward() -> Result<(), Error> {
        let cmd = "Application('Music').fastForward()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Skips current Track and plays next one.
    pub fn next_track() -> Result<(), Error> {
        let cmd = "Application('Music').nextTrack()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Plays if Player is currently Paused, Pauses if Player is currently Playing.
    pub fn playpause() -> Result<(), Error> {
        let cmd = "Application('Music').playpause()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Goes back to previous Track and plays it.
    pub fn previous_track() -> Result<(), Error> {
        let cmd = "Application('Music').previousTrack()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Rewinds current Track up until resuming or start of Track.
    pub fn rewind() -> Result<(), Error> {
        let cmd = "Application('Music').rewind()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Stops player, removing enqueued Tracks and currently playing Track.
    pub fn stop() -> Result<(), Error> {
        let cmd = "Application('Music').stop()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Fully Quits Apple Music.
    pub fn quit() -> Result<(), Error> {
        let cmd = "Application('Music').quit()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }

    /// Opens Apple Music app.
    pub fn run() -> Result<(), Error> {
        let cmd = "Application('Music').run()";

        let _ = ScriptController.execute(cmd, None);

        Ok(())
    }
}

/// Currently playing Repeat mode
#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum SongRepeatMode {
    OFF,
    ONE,
    ALL,
}
