use axum::response::IntoResponse;

pub async fn handler() -> impl IntoResponse {
    let spotify_client = crate::SPOTIFY_CLIENT.lock().await;
    let url = spotify_client.get_oauth_url();

    axum::response::Redirect::temporary(&url)
}
