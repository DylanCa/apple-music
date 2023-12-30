# Apple Music
_A Rust Library to control local Apple Music player_

## Development is still ongoing, package is not ready yet!
Currently available:
- ApplicationData
- Track
- AppCommands

## HowTo 
_(super lightweight, will update it real soon)_
```rust
let app_data = AppleMusic::get_application_data();
let track = AppleMusic::get_current_track();

AppleMusic::execute(AppCommands::PLAYPAUSE);

```