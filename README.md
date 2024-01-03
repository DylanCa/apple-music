# Apple Music
_A Rust Library to control local Apple Music player_

[![crates.io](https://img.shields.io/crates/v/apple-music.svg)](https://crates.io/crates/apple-music) [![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://crates.io/crates/apple-music)

## Development is still ongoing, package is not ready yet!
Currently available:
- ApplicationData
- Track
- Playlist + Playlist Tracks
- AppCommands

Available but buggy:
- Library & Playist tracks if they exceed ~900 tracks:
  - This seems to be a JavaScript limit, I need to look into it.


## HowTo 
_(super lightweight, will update it real soon)_
```rust
let app_data = AppleMusic::get_application_data();
let track = AppleMusic::get_current_track();

AppleMusic::execute(AppCommands::PLAYPAUSE);
```

## Next Steps before v1.0
- Finish to add remaining classes & methods:
  - `ADD()`
  - `EXPORT()`
  - `REFRESH()`
- Fully document and test everything
