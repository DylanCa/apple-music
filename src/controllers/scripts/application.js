function run(input) {
    const Music = Application("Music");
    const application = {};

    airplay_devices = [];
    Music.currentAirPlayDevices().forEach((device) => {
        airplay_devices.push({
            class: device.class(),
            id: device.id(),
            index: device.index(),
            name: device.name(),
            persistent_id: device.persistentID(),
            raw_properties: JSON.stringify(device.properties()),

            active: device.active(),
            available: device.available(),
            network_address: device.networkAddress(),
            protected: device.protected(),
            selected: device.selected(),
            supports_audio: device.supportsAudio(),
            supports_video: device.supportsVideo(),
            sound_volume: device.soundVolume(),
        });
    })

    if (airplay_devices.length > 0) {
        application.current_airplay_devices = airplay_devices;
    }

    application.airplay_enabled = Music.airplayEnabled();
    application.converting = Music.converting();
    eq_presets = [];
    Music.eqPresets().forEach((preset) => {
        eq_presets.push({
            class: preset.class(),
            id: preset.id(),
            index: preset.index(),
            name: preset.name(),
            raw_properties: JSON.stringify(preset.properties()),

            band1: preset.band1(),
            band2: preset.band2(),
            band3: preset.band3(),
            band4: preset.band4(),
            band5: preset.band5(),
            band6: preset.band6(),
            band7: preset.band7(),
            band8: preset.band8(),
            band9: preset.band9(),
            band10: preset.band10(),
            modifiable: preset.modifiable(),
            preamp: preset.preamp(),
            update_tracks: preset.updateTracks(),
        });
    });
    application.current_eq_presets = eq_presets;

    try { application.current_playlist = extract_playlist(Music.currentPlaylist()); }
    catch { application.current_playlist = null;}

    application.current_stream_title = Music.currentStreamTitle();
    application.current_stream_url = Music.currentStreamURL();

    visual = Music.currentVisual()
    application.current_visual = {
        class: visual.class(),
        id: visual.id(),
        index: visual.index(),
        name: visual.name(),
        raw_properties: JSON.stringify(visual.properties()),
    };

    encoder = Music.currentEncoder()
    application.current_encoder = {
        class: encoder.class(),
        id: encoder.id(),
        index: encoder.index(),
        name: encoder.name(),
        raw_properties: JSON.stringify(encoder.properties()),

        format: encoder.format(),
    };
    application.eq_enabled = Music.eqEnabled();
    application.fixed_indexing = Music.fixedIndexing();
    application.frontmost = Music.frontmost();
    application.full_screen = Music.fullScreen();
    application.name = Music.name();
    application.mute = Music.mute();
    application.player_position = Music.playerPosition();
    application.player_state = Music.playerState();

    playlists = []
    Music.playlists().forEach((playlist) => playlists.push(extract_playlist(playlist)))
    application.playlists = playlists;

    application.selection = JSON.stringify(Music.selection());
    application.shuffle_enabled = Music.shuffleEnabled();
    application.shuffle_mode = Music.shuffleMode();
    application.song_repeat = Music.songRepeat();
    application.sound_volume = Music.soundVolume();
    application.version = Music.version();

    visuals = []
    Music.visuals().forEach((visual) => {
        visuals.push({
            class: visual.class(),
            id: visual.id(),
            index: visual.index(),
            name: visual.name(),
            raw_properties: JSON.stringify(visual.properties()),
        });
    });

    application.visuals = visuals;
    application.visuals_enabled = Music.visualsEnabled();

    return JSON.stringify(application);
}

function extract_playlist(playlist) {
    parent = null;
    if ("parent" in playlist.properties()) {
        parent = extract_playlist(playlist.parent());
    }

    return {
        class: playlist.class(),
        id: playlist.id(),
        index: playlist.index(),
        name: playlist.name(),
        persistent_id: playlist.persistentID(),
        raw_properties: JSON.stringify(playlist.properties()),

        description: playlist.description(),
        disliked: playlist.disliked(),
        duration: playlist.duration(),
        loved: playlist.loved(),
        parent: parent,
        size: playlist.size(),
        special_kind: playlist.specialKind(),
        time: playlist.time(),
        visible: playlist.visible(),
    }

}