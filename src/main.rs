use dotenv::dotenv;
use std::env;
use tracing::{debug, error, warn};

use rspotify::model::PlaylistId;
use tokio::{join, runtime::Builder};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::spotify::downloader::spotify_bulk_download;

mod spotify;

fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spotisync=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let playlist_sync_task = async {
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
        };

        let download_loop = async {
            loop {
                spotify_bulk_download().await;

                if let Ok(url) = env::var("PLEX_REFRESH_URL") {
                    match make_get_request(&url).await {
                        Ok(_) => debug!("Request successful!"),
                        Err(err) => error!("Request failed: {}", err),
                    }
                } else {
                    warn!("PLEX_REFRESH_URL environment variable not set.");
                }

                // Sleep 24 hours
                tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60 * 24)).await;
            }
        };

        join!(playlist_sync_task, download_loop);
    });
}

pub async fn make_get_request(url: &str) -> anyhow::Result<()> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        anyhow::Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Received a non-successful status code: {}",
            response.status()
        ))
    }
}
