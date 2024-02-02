use axum::{self, routing::post, Router};
use dotenv::dotenv;

use downloader::DownloadQueue;
use lazy_static::lazy_static;
use tracing::{error, info};
use tracing_subscriber::{
    filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

mod downloader;
mod environment;
mod plex;
mod routes;

pub type AsyncDownloadQueue = std::sync::Arc<tokio::sync::Mutex<DownloadQueue>>;

lazy_static! {
    pub static ref DOWNLOAD_QUEUE: AsyncDownloadQueue =
        std::sync::Arc::new(tokio::sync::Mutex::new(DownloadQueue::new()));
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    start_download_loop();

    let port = crate::environment::get_port();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let host_uri = crate::environment::get_host_uri();

    info!("Listening on {}", host_uri);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let router = Router::new().route("/queue", post(routes::route_queue::post));
    axum::serve(listener, router).await.unwrap();
}

async fn get_next_item() -> Option<downloader::Item> {
    DOWNLOAD_QUEUE.lock().await.next()
}

fn start_download_loop() {
    tokio::spawn(async move {
        loop {
            if let Some(item) = get_next_item().await {
                let result = downloader::download_item(item).await;
                match result {
                    Ok(output) => {
                        info!("Downloaded item: {:?}", output);
                        crate::plex::refresh_plex_library().await;
                    }
                    Err(err) => {
                        error!("Failed to download item: {:?}", err);
                    }
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    });
}
