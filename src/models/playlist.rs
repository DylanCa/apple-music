use crate::controllers::script_controller::{ParamType, ScriptController};
use crate::models::error::Error;
use crate::models::track::Track;
use serde::Deserialize;
use serde_json::json;
use std::process::Output;

#[derive(Deserialize, Debug)]
pub enum SpecialKind {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "folder")]
    Folder,
    Genius,
    Library,
    Music,
    PurchasedMusic,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PlaylistClass {
    LibraryPlaylist,
    UserPlaylist,
    SubscriptionPlaylist,
    FolderPlaylist,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub class: PlaylistClass, // the class of the item
    pub id: i32,              // the id of the item
    pub index: i32,           // the index of the item in internal application order
    pub name: String,         // the name of the item

    #[serde(rename = "persistentID")]
    pub persistent_id: String, // the id of the item as a hexadecimal string. This id does not change over time.

    pub description: Option<String>, // the description of the playlist
    pub disliked: bool,              // is this playlist disliked?
    pub duration: Option<f32>,       // the total length of all tracks (in seconds)
    pub loved: bool,                 // is this playlist loved?
    pub parent: Option<Box<Playlist>>, // folder which contains this playlist (if any)
    pub size: Option<i64>,           // the total size of all tracks (in bytes)

    pub special_kind: Option<SpecialKind>, // special playlist kind
    pub time: Option<String>,              // the length of all tracks in MM:SS format
    pub tracks: Option<Vec<Track>>,        // Playlist's tracks
    pub visible: Option<bool>,             // is this playlist visible in the Source list?
}

impl Playlist {
    pub fn fetch_playlist_tracks(&mut self) -> Result<(), Error> {
        match ScriptController.execute_script::<Vec<Track>>(
            ParamType::PlaylistTracks,
            Some(self.id),
            None,
        ) {
            Ok(data) => {
                self.tracks = Some(data);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn search_for_tracks(&self, query: &str) -> Result<Option<Vec<Track>>, Error> {
        match ScriptController.execute_script::<Vec<Track>>(
            ParamType::SearchInPlaylist,
            Some(self.id),
            Some(query),
        ) {
            Ok(data) => Ok(Option::from(data)),
            Err(err) => Err(err),
        }
    }

    pub fn reveal_in_player(&self) -> Result<Output, Error> {
        let cmd = format!(
            "Application('Music').reveal(Application('Music').playlists.byId({}))",
            self.id
        );

        ScriptController.execute(cmd.as_str(), None)
    }

    pub fn download(&self) -> Result<Output, Error> {
        let cmd = format!(
            "Application('Music').download(Application('Music').playlists.byId({}))",
            self.id
        );

        ScriptController.execute(cmd.as_str(), None)
    }
}
