use rspotify::model::PlaylistId;

mod spotify;

#[tokio::main]
async fn main() {
    let spotify = spotify::auth::get_spotify_client().await;

    let source: PlaylistId = PlaylistId::from_id_or_uri("6cHhVGOS9UBamBzw53SQZL").unwrap();
    let target: PlaylistId = PlaylistId::from_id_or_uri("3KAGyeFZK1uDfet9hOd6gU").unwrap();

    spotify::playlist_sync::one_way_sync(spotify, source, target).await;
}
