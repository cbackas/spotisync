use futures_util::TryStreamExt;
use rspotify::{
    http::HttpError,
    model::{PlaylistId, PlaylistItem},
    prelude::BaseClient,
    AuthCodeSpotify,
};
use tracing::{info, warn};

pub async fn get_playlist_tracks(
    spotify: AuthCodeSpotify,
    playlist_id: &PlaylistId<'_>,
) -> anyhow::Result<Vec<PlaylistItem>> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        info!(
            "Getting tracks from playlist {} (attempt {})",
            playlist_id, attempt
        );

        let spotify_response = spotify
            .playlist_items(playlist_id.as_ref(), None, None)
            .try_collect()
            .await;

        match spotify_response {
            Ok(playlist_items) => return Ok(playlist_items),
            Err(rspotify::ClientError::Http(http_error)) => match *http_error {
                HttpError::StatusCode(response) => {
                    if response.status().as_u16() == 429 {
                        let default_delay = reqwest::header::HeaderValue::from_str("20").unwrap();
                        let retry_delay = response
                            .headers()
                            .get("Retry-After")
                            .unwrap_or(&default_delay);
                        let retry_delay = retry_delay.to_str().unwrap().parse::<u64>().unwrap();
                        warn!("Rate limited, waiting {} seconds", retry_delay);
                        std::thread::sleep(std::time::Duration::from_secs(retry_delay));
                        continue;
                    } else {
                        return Err(anyhow::Error::msg(format!(
                            "Error getting playlist tracks: {}",
                            response.status()
                        )));
                    }
                }
                _ => {
                    return Err(anyhow::Error::msg(format!(
                        "Error getting playlist tracks: {}",
                        http_error.to_string()
                    )))
                }
            },
            Err(e) => {
                return Err(anyhow::Error::msg(format!(
                    "Error getting playlist tracks: {}",
                    e
                )))
            }
        }
    }
}

// /**
// * Get all albums for an artist for a given AlbumType
// * Handles rate limiting by waiting the requested amount of time
// **/
// pub async fn get_artist_albums_by_type(
//     spotify: AuthCodeSpotify,
//     artist_id: ArtistId<'_>,
//     album_type: AlbumType,
// ) -> anyhow::Result<Vec<SimplifiedAlbum>> {
//     info!("Getting {:?} albums for artist {:?}", album_type, artist_id);
//
//     loop {
//         let spotify_response = spotify
//             .artist_albums(artist_id.clone(), Some(album_type), Some(Market::FromToken))
//             .try_collect()
//             .await;
//
//         match spotify_response {
//             Ok(albums) => return Ok(albums),
//             Err(rspotify::ClientError::Http(http_error)) => match *http_error {
//                 HttpError::StatusCode(response) => {
//                     if response.status().as_u16() == 429 {
//                         let default_delay = reqwest::header::HeaderValue::from_str("20").unwrap();
//                         let retry_delay = response
//                             .headers()
//                             .get("Retry-After")
//                             .unwrap_or(&default_delay);
//                         let retry_delay = retry_delay.to_str().unwrap().parse::<u64>().unwrap();
//                         warn!("Rate limited, waiting {} seconds", retry_delay);
//                         std::thread::sleep(std::time::Duration::from_secs(retry_delay));
//                         continue;
//                     } else {
//                         return Err(anyhow::Error::msg(format!(
//                             "Error getting artist albums: {}",
//                             response.status()
//                         )));
//                     }
//                 }
//                 _ => {
//                     return Err(anyhow::Error::msg(format!(
//                         "Error getting artist albums: {}",
//                         http_error.to_string()
//                     )))
//                 }
//             },
//             Err(e) => {
//                 return Err(anyhow::Error::msg(format!(
//                     "Error getting artist albums: {}",
//                     e
//                 )))
//             }
//         }
//     }
// }
//
// /**
// * Get all "Album" and "Single" albums for an artist
// **/
// pub async fn get_artist_albums(
//     spotify: AuthCodeSpotify,
//     artist_id: ArtistId<'_>,
// ) -> anyhow::Result<Vec<SimplifiedAlbum>> {
//     let albums_album =
//         get_artist_albums_by_type(spotify.clone(), artist_id.clone(), AlbumType::Album).await?;
//     let albums_singles =
//         get_artist_albums_by_type(spotify, artist_id.clone(), AlbumType::Single).await?;
//
//     let albums = albums_album
//         .into_iter()
//         .chain(albums_singles.into_iter())
//         .filter(|album| {
//             album.artists.len() == 1 && album.artists[0].id == Some(artist_id.clone_static())
//         })
//         .collect::<Vec<SimplifiedAlbum>>();
//
//     Ok(albums)
// }
