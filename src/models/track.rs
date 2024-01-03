use std::process::Output;
use serde::Deserialize;
use crate::controllers::script_controller::ScriptController;
use crate::models::artwork::Artwork;
use crate::models::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    User,
    Computed
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CloudStatus {
    Unknown,
    Purchased,
    Matched,
    Uploaded,
    Ineligible,
    Removed,
    Error,
    Duplicate,
    Subscription,
    Prerelease,

    #[serde(rename = "no longer available")]
    NoLongerAvailable,
    NotUploaded
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    Song,
    MusicVideo,
    Unknown
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub class: String, // the class of the item
    pub id: i32, // the id of the item
    pub index: i32, // the index of the item in internal application order
    pub name: String, // the name of the item
    #[serde(rename = "persistentID")]
    pub persistent_id: String, // the id of the item as a hexadecimal string. This id does not change over time.

    pub album: String, // The album name of the track
    pub album_artist: String, // The album artist of the track
    pub album_disliked: bool, // Is the album for this track disliked?
    pub album_loved: bool, // is the album for this track loved?
    pub album_rating: Option<i16>, // the rating of the album for this track (0 to 100)
    pub album_rating_kind: Option<Kind>, // the rating kind of the album rating for this track
    pub artworks: Option<Vec<Artwork>>, // the artist/source of the track
    pub bit_rate: Option<i16>, // the bit rate of the track (in kbps)
    pub bookmark: i8, // the bookmark time of the track in seconds
    pub bookmarkable: bool, // is the playback position for this track remembered?
    pub bpm: i16, // the tempo of this track in beats per minute
    pub category: String, // the category of the trac
    pub cloud_status: Option<CloudStatus>, // the iCloud status of the track
    pub comment: String, // freeform notes about the track
    pub compilation: bool, // is this track from a compilation album?
    pub composer: String, // the composer of the track

    #[serde(rename = "databaseID")]
    pub database_id: i32, // the common, unique ID for this track. If two tracks in different playlists have the same database ID, they are sharing the same data.
    pub date_added: String, // the date the track was added to the playlist
    pub description: String, // the description of the track
    pub disliked: bool, // is this track disliked?
    #[serde(rename = "downloaderAppleID")]
    pub downloader_apple_id: Option<String>, // the Apple ID of the person who downloaded this track
    pub downloader_name: Option<String>, // the name of the person who downloaded this track
    pub duration: f64, // the length of the track in seconds
    pub enabled: bool, // is this track checked for playback?
    #[serde(rename = "episodeID")]
    pub episode_id: Option<String>, // the episode ID of the track
    pub episode_number: i16, // the episode number of the track
    pub eq: String, // the name of the EQ preset of the track
    pub finish: f64, // the stop time of the track in seconds
    pub gapless: Option<bool>, // is this track from a gapless album?
    pub genre: String, // the music/audio genre (category) of the track
    pub grouping: String, // the grouping (piece) of the track. Generally used to denote movements within a classical work.
    pub kind: Option<String>, // a text description of the track
    pub long_description: Option<String>, // the long description of the track
    pub loved: bool, // is this track loved?
    pub lyrics: Option<String>, // the lyrics of the track
    pub media_kind: MediaKind, // the media kind of the track
    pub modification_date: Option<String>, // the modification date of the content of this track
    pub movement: Option<String>, // the movement name of the track
    pub movement_count: i16, // the total number of movements in the work
    pub movement_number: i16, // the index of the movement in the work
    pub played_count: i16, // number of times this track has been played
    pub played_date: Option<String>, // the date and time this track was last played
    #[serde(rename = "purchaserAppleID")]
    pub purchaser_apple_id: Option<String>, // the Apple ID of the person who purchased this track
    pub purchaser_name: Option<String>, // the name of the person who purchased this track
    pub rating: i16, // the rating of this track (0 to 100)
    pub rating_kind: Option<Kind>, // the rating kind of this track
    pub release_date: String, // the release date of this track
    pub sample_rate: Option<i32>, // the sample rate of the track (in Hz)
    pub season_number: Option<i16>, // the season number of the track
    pub shufflable: bool, // is this track included when shuffling?
    pub skipped_count: i16, // number of times this track has been skipped
    pub skipped_date: Option<String>, // the date and time this track was last skipped
    pub show: Option<String>, // the show name of the track
    pub sort_album: Option<String>, // override string to use for the track when sorting by album
    pub sort_artist: Option<String>, // override string to use for the track when sorting by artist
    pub sort_album_artist: Option<String>, // override string to use for the track when sorting by album artist
    pub sort_name: Option<String>, // override string to use for the track when sorting by name
    pub sort_composer: Option<String>, // override string to use for the track when sorting by composer
    pub sort_show: Option<String>, // override string to use for the track when sorting by show name
    pub size: Option<i64>, // the size of the track (in bytes)
    pub start: f64, // the start time of the track in seconds
    pub time: String, // the length of the track in MM:SS format
    pub track_count: i16, // the total number of tracks on the source album
    pub track_number: i16, // the index of the track on the source album
    pub unplayed: bool, // is this track unplayed?
    pub volume_adjustment: i16, // relative volume adjustment of the track (-100% to 100%)
    pub work: Option<String>, // the work name of the track
    pub year: i16, // the year the track was recorded/released
}

impl Track {
    pub fn reveal_in_player(&self) -> Result<Output, Error> {
        let cmd = format!("Application('Music').reveal(Application('Music').tracks.byId({}))", self.id);

        ScriptController.execute(cmd.as_str(), None)
    }
}
