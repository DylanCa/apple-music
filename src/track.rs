use crate::error::Error;
use crate::script_controller::ScriptController;
use serde::Deserialize;

/// Provides data related to a specific Track as well as its artworks.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    /// The class of the Track
    pub class: TrackKind,

    /// The id of the item
    pub id: i32,

    /// The index of the item in internal application order
    pub index: i32,

    /// The name of the item
    pub name: String,

    /// The id of the item as a hexadecimal string. This id does not change over time.
    #[serde(rename = "persistentID")]
    pub persistent_id: String,

    /// The album name of the track
    pub album: String,

    /// The album artist of the track
    pub album_artist: String,

    /// Is the album for this track disliked?
    pub album_disliked: bool,

    /// Is the album for this track loved?
    pub album_loved: bool,

    /// The rating of the album for this track (0 to 100)
    pub album_rating: Option<i16>,

    /// The rating kind of the album rating for this track
    pub album_rating_kind: Option<Kind>,

    /// The artist/source of the track
    pub artworks: Option<Vec<Artwork>>,

    /// The bit rate of the track (in kbps)
    pub bit_rate: Option<i16>,

    /// The bookmark time of the track in seconds
    pub bookmark: i8,

    /// Is the playback position for this track remembered?
    pub bookmarkable: bool,

    /// The tempo of this track in beats per minute
    pub bpm: i16,

    /// The category of the track
    pub category: String,

    /// The iCloud status of the track
    pub cloud_status: Option<CloudStatus>,

    /// Freeform notes about the track
    pub comment: String,

    /// Is this track from a compilation album?
    pub compilation: bool,

    /// The composer of the track
    pub composer: String,

    /// The common, unique ID for this track. If two tracks in different playlists have the same database ID, they are sharing the same data.
    #[serde(rename = "databaseID")]
    pub database_id: i32,

    /// The date the track was added to the playlist
    pub date_added: String,

    /// The description of the track
    pub description: String,

    /// Is this track disliked?
    pub disliked: bool,

    /// The Apple ID of the person who downloaded this track
    #[serde(rename = "downloaderAppleID")]
    pub downloader_apple_id: Option<String>,

    /// The name of the person who downloaded this track
    pub downloader_name: Option<String>,

    /// The length of the track in seconds
    pub duration: f64,

    /// Is this track checked for playback?
    pub enabled: bool,

    /// The episode ID of the track
    #[serde(rename = "episodeID")]
    pub episode_id: Option<String>,

    /// The episode number of the track
    pub episode_number: i16,

    /// The name of the EQ preset of the track
    pub eq: String,

    /// The stop time of the track in seconds
    pub finish: f64,

    /// Is this track from a gapless album?
    pub gapless: Option<bool>,

    /// The music/audio genre (category) of the track
    pub genre: String,

    /// The grouping (piece) of the track. Generally used to denote movements within a classical work.
    pub grouping: String,

    /// A text description of the track
    pub kind: Option<String>,

    /// The long description of the track
    pub long_description: Option<String>,

    /// Is this track loved?
    pub loved: bool,

    /// The lyrics of the track
    pub lyrics: Option<String>,

    /// The media kind of the track
    pub media_kind: MediaKind,

    /// The modification date of the content of this track
    pub modification_date: Option<String>,

    /// The movement name of the track
    pub movement: Option<String>,

    /// The total number of movements in the work
    pub movement_count: i16,

    /// The index of the movement in the work
    pub movement_number: i16,

    /// Number of times this track has been played
    pub played_count: i16,

    /// The date and time this track was last played
    pub played_date: Option<String>,

    /// The Apple ID of the person who purchased this track
    #[serde(rename = "purchaserAppleID")]
    pub purchaser_apple_id: Option<String>,

    /// The name of the person who purchased this track
    pub purchaser_name: Option<String>,

    /// The rating of this track (0 to 100)
    pub rating: i16,

    /// The rating kind of this track
    pub rating_kind: Option<Kind>,

    /// The release date of this track
    pub release_date: String,

    /// The sample rate of the track (in Hz)
    pub sample_rate: Option<i32>,

    /// The season number of the track
    pub season_number: Option<i16>,

    /// Is this track included when shuffling?
    pub shufflable: bool,

    /// Number of times this track has been skipped
    pub skipped_count: i16,

    /// The date and time this track was last skipped
    pub skipped_date: Option<String>,

    /// The show name of the track
    pub show: Option<String>,

    /// Override string to use for the track when sorting by album
    pub sort_album: Option<String>,

    /// Override string to use for the track when sorting by artist
    pub sort_artist: Option<String>,

    /// Override string to use for the track when sorting by album artist
    pub sort_album_artist: Option<String>,

    /// Override string to use for the track when sorting by name
    pub sort_name: Option<String>,

    /// Override string to use for the track when sorting by composer
    pub sort_composer: Option<String>,

    /// Override string to use for the track when sorting by show name
    pub sort_show: Option<String>,

    /// The size of the track (in bytes)
    pub size: Option<i64>,

    /// The start time of the track in seconds
    pub start: f64,

    /// The length of the track in MM:SS format
    pub time: String,

    /// The total number of tracks on the source album
    pub track_count: i16,

    /// The index of the track on the source album
    pub track_number: i16,

    /// Is this track unplayed?
    pub unplayed: bool,

    /// Relative volume adjustment of the track (-100% to 100%)
    pub volume_adjustment: i16,

    /// The work name of the track
    pub work: Option<String>,

    /// The year the track was recorded/released
    pub year: i16,
}

impl Track {
    /// Reveals and selects Track in Apple Music.
    pub fn reveal_in_player(&self) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').reveal(Application('Music').tracks.byId({}))",
            self.id
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Triggers a download on Apple Music Player for the Track.
    pub fn download(&self) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').download(Application('Music').tracks.byId({}))",
            self.id
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Loves / "Unloves" a Track.
    pub fn set_loved(&self, value: bool) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').tracks.byId({}).loved = {}",
            self.id, value
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }

    /// Dislikes / "Undislikes" a Track.
    pub fn set_disliked(&self, value: bool) -> Result<(), Error> {
        let cmd = format!(
            "Application('Music').tracks.byId({}).disliked = {}",
            self.id, value
        );

        let _ = ScriptController.execute(cmd.as_str(), None);

        Ok(())
    }
}

/// Data for a given Artwork.
#[derive(Deserialize, Debug)]
pub struct Artwork {
    /// The class of the item.
    pub class: String,

    /// Data for the artwork, in the form of a picture.
    pub data: Option<String>,

    /// Description of artwork as a string.
    pub description: Option<String>,

    /// Was this artwork downloaded by Apple Music ?
    pub downloaded: bool,

    /// The data format for this piece of artwork.
    pub format: Option<String>,

    /// Kind or purpose of this piece of artwork.
    pub kind: i32,

    /// Data for this artwork, in original format.
    pub raw_data: String,
}

/// Type of Rating: User-made or Computed.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    User,
    Computed,
}

/// iCloud status for Track.
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
    NotUploaded,
}

/// Type of Media: Song, MusicVideo or Unknown.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MediaKind {
    Song,
    MusicVideo,
    Unknown,
}

/// Type of Track: From an URL, a File, or Shared.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TrackKind {
    SharedTrack,
    FileTrack,
    UrlTrack,
}
