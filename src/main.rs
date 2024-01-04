use apple_music::AppleMusic;
use std::time::Instant;
use std::env;

pub fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let now_all = Instant::now();

    let mut track = AppleMusic::get_current_track().unwrap();

    println!("Elapsed - get_all_library_tracks(): {:.2?}", now_all.elapsed());
    println!("{:#?}", &track.artworks.as_ref());
    track.fetch_artworks().expect("");
    println!("{:#?}", &track.artworks.unwrap().len());


    AppleMusic::set_sound_volume(50).expect("");

    let app_data = AppleMusic::get_application_data().unwrap();
    let playlist = app_data.current_playlist.unwrap();

    println!("{:#?}", playlist.name);
    println!("{:#?}", AppleMusic::get_current_track().unwrap().name);

    let playlist = &app_data.playlists.unwrap()[4];
    println!("{:#?}", playlist.name);

    AppleMusic::play_playlist(playlist).expect("");

    playlist.reveal_in_player().expect("");

    AppleMusic::get_current_track().unwrap().reveal_in_player().expect("");

    println!("{:#?}", AppleMusic::get_current_track().unwrap().name);

    println!("Elapsed - Everything: {:.2?}", now_all.elapsed());
}
