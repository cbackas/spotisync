use std::collections::HashSet;

use rspotify::{
    model::{PlayableItem, PlaylistId},
    prelude::Id,
    AuthCodeSpotify,
};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tracing::{debug, error, info};

use super::functions::{get_artist_albums, get_playlist_tracks};

pub async fn playlist_album_dump(spotify: AuthCodeSpotify, playlist_id: &PlaylistId<'_>) {
    info!("Starting album dump from playlist {}", playlist_id);

    let playlist_tracks = get_playlist_tracks(spotify.clone(), &playlist_id)
        .await
        .unwrap_or(Vec::new());

    let mut unique_artists = HashSet::new();

    for track in &playlist_tracks {
        if let Some(PlayableItem::Track(playable_track)) = &track.track {
            for artist in &playable_track.album.artists {
                unique_artists.insert((artist.id.clone().unwrap(), artist.name.clone()));
            }
        }
    }

    for (artist_id, artist_name) in unique_artists {
        let albums = get_artist_albums(spotify.clone(), artist_id)
            .await
            .unwrap_or(Vec::new());
        for album in albums {
            if let Some(id) = album.id {
                download_spotify_thing(&id.url(), format!("{} - {}", artist_name, album.name))
                    .await;

                // sleep for 20 seconds to avoid rate limiting
                debug!("Sleeping before next download...");
                tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
            };
        }
    }
}

async fn download_spotify_thing(id: &str, friendly_name: String) {
    info!("Starting download of {}...", friendly_name);
    debug!("Downloading Spotify tracks from {}...", id);

    let mut command = Command::new("down_on_spot");
    command.arg(id);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    match command.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let stderr = child.stderr.take().expect("Failed to capture stderr");

            let mut stdout_reader = BufReader::new(stdout).lines();
            let mut stderr_reader = BufReader::new(stderr).lines();

            tokio::select! {
                _ = async {
                    while let Some(line) = stdout_reader.next_line().await.unwrap() {
                        debug!("{}", line);
                    }
                } => {},
                _ = async {
                    while let Some(line) = stderr_reader.next_line().await.unwrap() {
                        error!("{}", line);
                    }
                } => {},
            }

            let status = child.wait().await.expect("Failed to wait on child");

            if status.success() {
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
