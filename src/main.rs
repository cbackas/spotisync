use std::env;

use rspotify::model::PlaylistId;

mod spotify;

#[tokio::main]
async fn main() {
    let spotify = spotify::auth::get_spotify_client().await;

    let source_playlist_id: String = match env::var_os("SYNC_SOURCE_PLAYLIST_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("Missing SYNC_SOURCE_PLAYLIST_ID env var"),
    };
    let source_playlist_id: PlaylistId = PlaylistId::from_id_or_uri(&source_playlist_id)
        .expect("Failed to parse source playlist id");

    let target_playlist_id: String = match env::var_os("SYNC_TARGET_PLAYLIST_ID") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("Missing SYNC_TARGET_PLAYLIST_ID env var"),
    };
    let target_playlist_id: PlaylistId = PlaylistId::from_id_or_uri(&target_playlist_id)
        .expect("Failed to parse target playlist id");

    let continous: bool = match env::var_os("CONTINUOUS_SYNC") {
        Some(v) => v.into_string().unwrap() == "true",
        None => false,
    };

    loop {
        spotify::playlist_sync::one_way_sync(
            spotify.clone(),
            &source_playlist_id,
            &target_playlist_id,
        )
        .await;

        if !continous {
            break;
        }

        // sleep 40 seconds
        std::thread::sleep(std::time::Duration::from_secs(40));
    }
}
