use std::process::Command;

use rspotify::{
    model::{PlayableItem, PlaylistId},
    AuthCodeSpotify,
};
use tracing::{debug, error, info};

use super::functions::{get_artist_albums, get_playlist_tracks};

pub async fn playlist_album_dump(spotify: AuthCodeSpotify, playlist_id: &PlaylistId<'_>) {
    info!("Starting album dump from playlist {}", playlist_id);

    let playlist_tracks = get_playlist_tracks(spotify.clone(), &playlist_id)
        .await
        .unwrap_or(Vec::new());

    for track in playlist_tracks {
        if let Some(PlayableItem::Track(playable_track)) = track.track {
            let artists = playable_track.album.artists;
            for artist in artists {
                let artist_id = artist.id.unwrap();
                let albums = get_artist_albums(spotify.clone(), artist_id)
                    .await
                    .unwrap_or(Vec::new());
                for album in albums {
                    if let Some(href) = album.href {
                        download_spotify_thing(href, format!("{} - {}", artist.name, album.name));
                    };
                }
            }
        }
    }
}

fn download_spotify_thing(url: String, friendly_name: String) {
    info!("Starting download of {}", friendly_name);
    debug!("Downloading Spotify tracks from {}", url);

    let mut command = Command::new("down_on_spot");
    command.arg(url);

    match command.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            debug!("STDOUT: {}", stdout);
            debug!("STDERR: {}", stderr);

            if output.status.success() {
                info!("Successfully downloaded {}", friendly_name);
            } else {
                error!("Failed to download {}", friendly_name);
            }
        }
        Err(error) => {
            error!("Failed to execute command: {}", error);
        }
    }
}
