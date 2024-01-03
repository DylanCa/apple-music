function run(input) {
    let params = JSON.parse(input);

    switch (params['param_type']) {
        case "allTracks":
            return all_tracks();

        case "currentTrack":
            let track = extract_track(Application("Music").currentTrack());
            return JSON.stringify(track);

        case "playlistById":
            let playlist = Application("Music").playlists.byId(params["id"]);
            let data = extract_playlist(playlist);
            return JSON.stringify(data);

        case "playlistTracks":
            return playlist_tracks(params["id"]);

        case "applicationData":
            return application_data();

        case "searchInPlaylist":
            return search_in_playlist(params["id"], params["query"]);
    }
}

function application_data() {
    const Music = Application("Music");
    const application = Music.properties();

    application.currentAirplayDevices = [];
    Music.currentAirPlayDevices().forEach((device) => {
        application.currentAirplayDevices.push(device.properties());
    })

    application.eqPresets = [];
    Music.eqPresets().forEach((preset) => {
        application.eqPresets.push(preset.properties());
    });

    try {
        application.currentPlaylist = extract_playlist(Music.currentPlaylist());
    } catch {
        application.currentPlaylist = null;
    }

    application.selection = []

    Music.selection().forEach((track) => {
        try {
            application.selection.push(extract_track(track));
        } catch { /* continue loop */
        }
    });

    application.currentEncoder = Music.currentEncoder().properties();

    application.playlists = []
    Music.playlists().forEach((playlist) => application.playlists.push(extract_playlist(playlist)));

    application.currentVisual = Music.currentVisual().properties();

    application.visuals = []
    Music.visuals().forEach((visual) => {
        application.visuals.push(visual.properties());
    });

    return JSON.stringify(application);
}

function extract_track(current_track) {
    let track = current_track.properties();
    try {
        let artworks = []

        current_track.artworks().forEach((artwork) => {
            let json = artwork.properties();
            json.raw_data = artwork.rawData();
            artworks.push(json)
        });

        track.artworks = artworks;
    } catch { /* do nothing */
    }
    return track;
}

function extract_playlist(playlist) {
    let data = playlist.properties();

    try {
        data.parent = extract_playlist(playlist.parent());
    } catch { /* do nothing */
    }

    return data;
}

function all_tracks() {
    let tracks = []
    Application("Music").tracks().forEach((track) => {
        try {
            tracks.push(extract_track(track));
        } catch { /* continue loop */
        }
    });

    return JSON.stringify(tracks);
}

function search_in_playlist(id, query) {
    let results = Application("Music").playlists.byId(id).search({for: query});

    let tracks = []
    results.forEach((track) => tracks.push(extract_track(track)));
    return JSON.stringify(tracks)
}

function playlist_tracks(id) {
    const playlist = Application("Music").playlists.byId(id)

    let tracks = []
    playlist.tracks().forEach((track) => tracks.push(extract_track(track)));
    return JSON.stringify(tracks);
}
