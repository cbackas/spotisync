use dotenv::dotenv;
use std::env;

use rspotify::model::PlaylistId;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod spotify;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spotisync=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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

    loop {
        spotify::playlist_sync::one_way_sync(
            spotify.clone(),
            &source_playlist_id,
            &target_playlist_id,
        )
        .await;

        // sleep 40 seconds
        tokio::time::sleep(tokio::time::Duration::from_secs(40)).await;
    }
}
