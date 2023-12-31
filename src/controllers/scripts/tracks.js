function run(input) {
    let param = JSON.parse(input)['param'];
    if(param == "allTracks") {
        const Music = Application("Music");
        tracks = []
        Music.tracks().forEach((track) => tracks.push(extract_track(track)))
        return JSON.stringify(tracks);
    } else if (param == "playlistTracks" ) {
        let id = JSON.parse(input)['id'];
        const Music = Application("Music");
        const playlist = Music.sources.byId(64).playlists.byId(41554)

        tracks = []
        playlist.tracks().forEach((track) => tracks.push(extract_track(track)));
        return JSON.stringify(tracks);
    } else {
        const Music = Application("Music");
        let track = extract_track(Music.currentTrack());
        return JSON.stringify(track);
    }
}

function extract_track(current_track) {
    const track = {};

    artworks = [];

    current_track.artworks().forEach((artwork) => {
        artworks.push({
            class: artwork.class(),
            raw_properties: JSON.stringify(artwork.properties()),

            data: artwork.data(),
            description: artwork.description(),
            downloaded: artwork.downloaded(),
            format: artwork.format(),
            kind: artwork.kind(),
            raw_data: artwork.rawData(),
        });
    });

    if (artworks.length > 0) {
        track.artworks = artworks;
    }

    track.album = current_track.album();
    track.album_artist = current_track.albumArtist();
    track.album_disliked = current_track.albumDisliked();
    track.album_loved = current_track.albumLoved();
    track.album_rating = current_track.albumRating();
    track.album_rating_kind = current_track.albumRatingKind();
    track.artist = current_track.artist();
    track.bit_rate = current_track.bitRate();
    track.bookmark = current_track.bookmark();
    track.bookmarkable = current_track.bookmarkable();
    track.bpm = current_track.bpm();
    track.category = current_track.category();
    track.class = current_track.class();
    track.cloud_status = current_track.cloudStatus();
    track.comment = current_track.comment();
    track.compilation = current_track.compilation();
    track.composer = current_track.composer();
    track.database_id = current_track.databaseID();
    track.date_added = current_track.dateAdded();
    track.description = current_track.description();
    track.disc_count = current_track.discCount();
    track.disc_number = current_track.discNumber();
    track.disliked = current_track.disliked();
    track.downloader_apple_id = current_track.downloaderAppleID();
    track.downloader_name = current_track.downloaderName();
    track.duration = current_track.duration();
    track.enabled = current_track.enabled();
    track.episode_id = current_track.episodeID();
    track.episode_number = current_track.episodeNumber();
    track.eq = current_track.eq();
    track.finish = current_track.finish();
    track.gapless = current_track.gapless();
    track.genre = current_track.genre();
    track.grouping = current_track.grouping();
    track.id = current_track.id();
    track.index = current_track.index();
    track.kind = current_track.kind();
    track.long_description = current_track.longDescription();
    track.loved = current_track.loved();
    track.lyrics = current_track.lyrics();
    track.media_kind = current_track.mediaKind();
    track.modification_date = current_track.modificationDate();
    track.movement = current_track.movement();
    track.movement_count = current_track.movementCount();
    track.movement_number = current_track.movementNumber();
    track.name = current_track.name();
    track.persistent_id = current_track.persistentID();
    track.played_count = current_track.playedCount();
    track.played_date = current_track.playedDate();
    track.raw_properties = JSON.stringify(current_track.properties());
    track.purchaser_apple_id = current_track.purchaserAppleID();
    track.purchaser_name = current_track.purchaserName();
    track.rating = current_track.rating();
    track.rating_kind = current_track.ratingKind();
    track.release_date = current_track.releaseDate();
    track.sample_rate = current_track.sampleRate();
    track.season_number = current_track.seasonNumber();
    track.shufflable = current_track.shufflable();
    track.skipped_count = current_track.skippedCount();
    track.skipped_date = current_track.skippedDate();
    track.show = current_track.show();
    track.sort_album = current_track.sortAlbum();
    track.sort_artist = current_track.sortArtist();
    track.sort_album_artist = current_track.sortAlbumArtist();
    track.sort_name = current_track.sortName();
    track.sort_composer = current_track.sortComposer();
    track.sort_show = current_track.sortShow();
    track.size = current_track.size();
    track.start = current_track.start();
    track.time = current_track.time();
    track.track_count = current_track.trackCount();
    track.track_number = current_track.trackNumber();
    track.unplayed = current_track.unplayed();
    track.volume_adjustment = current_track.volumeAdjustment();
    track.work = current_track.work();
    track.year = current_track.year();

    return track;
}