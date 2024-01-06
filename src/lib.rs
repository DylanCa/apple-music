//! # Apple Music
//! _A Rust Library to fully control local MacOS Apple Music player._
//!
//! [![crates.io](https://img.shields.io/crates/v/apple-music.svg)](https://crates.io/crates/apple-music)
//! [![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://crates.io/crates/apple-music)
//! [![docs.rs](https://img.shields.io/docsrs/apple-music)](https://docs.rs/apple-music/latest)
//!
//! This crate provides a convenient way of controlling a MacOS Apple Music player, fully through Rust code.
//! The logic behind this crate relies on Apple's scripting APIs through [`osascript` CLI](https://ss64.com/mac/osascript.html) and `JavaScript` scripts.
//!
//!
//! ## Example
//! No need to create an instance to use the crate:
//! ```rust
//! let playlist = &AppleMusic::get_playlist_by_id(1234).unwrap();
//! AppleMusic::play_playlist(playlist); // Apple Music player starts playing provided Playlist.
//!
//! AppleMusic::set_shuffle(true); // Shuffle is now enabled on currently playing Playlist.
//!
//! let track = playlist.fetch_playlist_tracks().unwrap()[5];
//! AppleMusic::play_track(track); // Apple Music player starts playing provided Track.
//!
//! let current_track = AppleMusic::get_current_track().unwrap();
//! println!("{}", current_track.name()); // "An awesome song!"
//!
//! println!("{}", current_track.artwork_url()); // Prints the direct url for the Artwork of the Track.
//!
//! current_track.set_loved(true); // Track is now loved!
//!
//! AppleMusic::next_track(); // Goes to next track.
//!
//! let current_track = AppleMusic::get_current_track().unwrap();
//! current_track.reveal_in_player(); // Track is revealed and selected on Apple Music player.
//!
//! current_track.set_disliked(true); // Track is now disliked!
//!
//! AppleMusic::set_sound_volume(15); // Sets Player volume to 15.
//!
//! playlist.download(); // Playlist is being downloaded on Apple Music player.
//!
//! AppleMusic::quit(); // Quit Apple Music application on Mac.
//! ```
//!
//!
//! ## Limitations
//! ### Code-related
//! Currently, `playlist.fetch_playlist_tracks()` and `AppleMusic::get_all_library_tracks()` are failing when fetching more than 900 tracks due to a JavaScript limitation when stringifying an object.
//!
//! ### Platforms
//! This crate only works on MacOs, and has only been tested with macOS 13.4.1 and Apple Music 1.3.5.
//!
//! I would be more than happy provide support for other version of MacOs / Apple Music, do not hesitate to open an issue if you are facing failures!

mod script_controller;

mod error;
mod track;

mod apple_music;
mod application_data;
mod playlist;

pub use apple_music::*;
pub use application_data::*;
pub use playlist::*;
pub use track::*;
