use axum::{extract::Query, response::IntoResponse};
use tracing::info;

use crate::{downloader::Item, DOWNLOAD_QUEUE};

pub async fn post(query: Query<Item>) -> impl IntoResponse {
    info!("Recieved queue request: {:?}", query);

    let mut item = query.0;
    // Remove the ?si= part of the id if it exists
    item.id = match item.id.contains("?si=") {
        true => item.id.split("?si=").collect::<Vec<&str>>()[0].to_string(),
        false => item.id,
    };

    DOWNLOAD_QUEUE.lock().await.add(item);

    axum::response::Json("your mother")
}
