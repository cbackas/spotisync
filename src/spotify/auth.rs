use std::path::PathBuf;

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};

pub async fn get_spotify_client() -> AuthCodeSpotify {
    let config = Config {
        cache_path: PathBuf::from("./cache/spotify_token.json"),
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let creds = Credentials::from_env()
        .expect("Missing RSPOTIFY_CLIENT_ID RSPOTIFY_CLIENT_SECRET env vars");

    let oauth = OAuth {
        redirect_uri: "http://localhost:8100/callback".to_string(),
        scopes: scopes!("playlist-modify-public", "playlist-modify-private", "playlist-read-private"),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify
        .prompt_for_token(&url)
        .await
        .expect("couldn't authenticate successfully");

    return spotify;
}
