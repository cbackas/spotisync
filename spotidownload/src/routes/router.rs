use axum::{routing::get, Router};

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(super::route_queue::handler))
        .route("/spotify/callback", get(super::route_queue::handler))
        .route("/spotify/auth", get(super::route_spotify_auth::handler))
}
