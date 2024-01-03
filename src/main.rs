use apple_music::AppleMusic;
use log::debug;
use std::time::Instant;
use std::{env, thread, time};

pub fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let now_all = Instant::now();

    /* let mut playlist = &mut AppleMusic::get_playlist_by_id(43141).unwrap();

        playlist.fetch_playlist_tracks();
        println!("Elapsed - fetch_tracks(): {:.2?}", now_all.elapsed());
        println!("{:#?}", playlist.tracks.as_ref().unwrap().len());
    */

    AppleMusic::set_sound_volume(50);

    /* println!("{:#?}", playlist.name);
    println!("{:#?}", AppleMusic::get_current_track().unwrap().name);

    let playlist = &AppleMusic::get_application_data()
        .unwrap()
        .playlists
        .unwrap()[9];
    println!("{:#?}", playlist.name);
    AppleMusic::play_playlist(playlist);
    playlist.reveal_in_player();
    AppleMusic::get_current_track().unwrap().reveal_in_player();
    println!("{:#?}", AppleMusic::get_current_track().unwrap().name);
    AppleMusic::play_track(&track);

    let mut playlist = AppleMusic::get_application_data()
        .unwrap()
        .current_playlist
        .unwrap();
    println!("{:#?}", playlist.name); */

    println!("Elapsed - Everything: {:.2?}", now_all.elapsed());
}
