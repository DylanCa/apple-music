use serde::Deserialize;
use crate::models::playlist::Playlist;

#[derive(Deserialize, Debug)]
pub struct ApplicationData {
    pub airplay_enabled: Option<bool>,
    // is AirPlay currently enabled?
    pub converting: Option<bool>,
    // is a track currently being converted?
    pub current_airplay_devices: Vec<AirplayDevice>,
    // the currently selected AirPlay device(s)
    pub current_encoder: Encoder,
    // the currently selected encoder (MP3, AIFF, WAV, etc.)
    pub current_eq_presets: Vec<EqPreset>,
    // the currently selected equalizer preset
    pub current_playlist: Option<Playlist>, // the playlist containing the currently targeted track
    pub current_stream_title: Option<String>,
    // the name of the current track in the playing stream (provided by streaming server)
    pub current_stream_url: Option<String>,
    // the URL of the playing stream or streaming web site (provided by streaming server)
    pub current_visual: Visual,
    // the currently selected visual plug-in
    pub eq_enabled: bool,
    // is the equalizer enabled?
    pub fixed_indexing: bool,
    // true if all AppleScript track indices should be independent of the play order of the owning playlist.
    pub frontmost: bool,
    // is this the active application?
    pub full_screen: bool,
    // is the application using the entire screen?
    pub name: Option<String>,
    // the name of the application
    pub mute: bool,
    // has the sound output been muted?
    pub player_position: Option<f64>,
    // : the player’s position within the currently playing track in seconds.
    pub player_state: Option<PlayerState>,
    // is the player stopped, paused, or playing?
    pub selection: Option<String>,
    // TODO: Implement - the selection visible to the user
    pub shuffle_enabled: bool,
    // are songs played in random order?
    pub shuffle_mode: ShuffleMode,
    // the playback shuffle mode
    pub song_repeat: SongRepeat, // (off/‌one/‌all) : the playback repeat mode
    pub sound_volume: i8,
    // the sound output volume (0 = minimum, 100 = maximum)
    pub version: Option<String>,
    // the version of the application
    pub visuals: Vec<Visual>,
    // list of visuals
    pub visuals_enabled: bool, // are visuals currently being displayed?
}

#[derive(Deserialize, Debug)]
pub struct AirplayDevice {
    pub class: String,
    // the class of the item
    pub id: i32,
    // the id of the item
    pub index: i32,
    // the index of the item in internal application order
    pub name: String,
    // the name of the item
    pub persistent_id: String,
    // the id of the item as a hexadecimal string. This id does not change over time.
    pub raw_properties: String, // Every property of the item

    pub active: Option<bool>,
    // is the device currently being played to?
    pub available: Option<bool>,
    // is the device currently available?
    // pub kind (computer/‌AirPort Express/‌Apple TV/‌AirPlay device/‌Bluetooth device/‌HomePod/‌unknown, r/o) : the kind of the device
    pub network_address: Option<String>,
    // the network (MAC) address of the device
    pub protected: Option<bool>,
    // is the device password- or passcode-protected?
    pub selected: bool,
    //is the device currently selected?
    pub supports_audio: Option<bool>,
    //does the device support audio playback?
    pub supports_video: Option<bool>,
    //does the device support video playback?
    pub sound_volume: i8, //the output volume for the device (0 = minimum, 100 = maximum)
}

#[derive(Deserialize, Debug)]
pub struct Encoder {
    pub class: String,
    // the class of the item
    pub id: i32,
    // the id of the item
    pub index: i32,
    // the index of the item in internal application order
    pub name: String,
    // the name of the item
    pub raw_properties: String, // Every property of the item

    pub format: String, // the data format created by the encoder
}

#[derive(Deserialize, Debug)]
pub struct EqPreset {
    pub class: String,
    // the class of the item
    pub id: i32,
    // the id of the item
    pub index: i32,
    // the index of the item in internal application order
    pub name: String,
    // the name of the item
    pub raw_properties: String, // Every property of the item

    pub band1: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band2: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band3: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band4: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band5: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band6: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band7: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band8: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band9: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub band10: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub modifiable: Option<bool>,
    // can this preset be modified?
    pub preamp: f32,
    // the equalizer 32 Hz band level (-12.0 dB to +12.0 dB)
    pub update_tracks: bool, // should tracks which refer to this preset be updated when the preset is renamed or deleted?
}

#[derive(Deserialize, Debug)]
pub struct Visual {
    pub class: String,
    // the class of the item
    pub id: i32,
    // the id of the item
    pub index: i32,
    // the index of the item in internal application order
    pub name: String,
    // the name of the item
    pub raw_properties: String, // Every property of the item
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
    FastForwarding,
    Rewinding,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ShuffleMode {
    Songs,
    Albums,
    Groupings,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SongRepeat {
    Off,
    On,
    All,
}
