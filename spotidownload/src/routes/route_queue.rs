use axum::{extract::Query, response::IntoResponse};
use tracing::info;

use crate::{downloader::Item, DOWNLOAD_QUEUE};

pub async fn post(query: Query<Item>) -> impl IntoResponse {
    info!("Recieved queue request: {:?}", query);

    let mut queue = DOWNLOAD_QUEUE.lock().await;
    queue.add(query.0);

    info!("Queue: {:?}", queue);

    axum::response::Json("your mother")
}
