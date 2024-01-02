use std::env;

pub async fn refresh_plex_library() {
    if let Ok(url) = env::var("PLEX_REFRESH_URL") {
        let response = reqwest::get(url).await;
        match response {
            Ok(_) => {
                println!("Refreshed Plex library");
            }
            Err(err) => {
                println!("Failed to refresh Plex library: {:?}", err);
            }
        }
    }
}
