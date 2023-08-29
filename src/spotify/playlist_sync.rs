use rspotify::{
    model::{FullTrack, PlayableItem, PlaylistId, PlaylistItem},
    prelude::*,
    AuthCodeSpotify,
};
use tracing::{error, info};

use crate::spotify::functions::get_playlist_tracks;

// takes in two spotify playlist ids and then syncs any songs that are in the source playlist but not in the target playlist
pub async fn one_way_sync(
    spotify: AuthCodeSpotify,
    source_playlist_id: &PlaylistId<'_>,
    target_playlist_id: &PlaylistId<'_>,
) {
    // this would be a waste of api calls
    if source_playlist_id == target_playlist_id {
        panic!("Source and target playlist are the same");
    }

    // fetch all tracks in both playlists
    let source_playlist: Vec<PlaylistItem> =
        match get_playlist_tracks(spotify.clone(), source_playlist_id).await {
            Ok(playlist) => playlist,
            Err(e) => {
                error!("Error getting source playlist: {}", e);
                Vec::new()
            }
        };
    let target_playlist: Vec<PlaylistItem> =
        match get_playlist_tracks(spotify.clone(), target_playlist_id).await {
            Ok(playlist) => playlist,
            Err(e) => {
                error!("Error getting target playlist: {}", e);
                Vec::new()
            }
        };

    // determine the tracks that are in the source playlist but not in the target playlist
    let unsynced_tracks: Vec<PlayableId> = get_unsynced_tracks(source_playlist, target_playlist)
        .into_iter()
        .map(|track| PlayableId::Track(track.id.unwrap()))
        .collect();

    if unsynced_tracks.is_empty() {
        info!("No tracks to sync");
        return;
    }

    // have spotify add the new tracks to the target playlist
    let sync_result = spotify
        .playlist_add_items(target_playlist_id.as_ref(), unsynced_tracks, None)
        .await;

    match sync_result {
        Ok(_) => info!("Successfully synced tracks to target playlist"),
        Err(e) => error!("Error syncing playlists: {}", e),
    }
}

// returns true if the track is in the provided list of playlist items
fn track_in_playlist(track: &FullTrack, playlist: &[PlaylistItem]) -> bool {
    playlist.iter().any(|playlist_item| {
        if let Some(PlayableItem::Track(target_track)) = &playlist_item.track {
            target_track.id == track.id
        } else {
            false
        }
    })
}

// returns a list of tracks that are in the source playlist but not in the target playlist
fn get_unsynced_tracks(
    source_playlist: Vec<PlaylistItem>,
    target_playlist: Vec<PlaylistItem>,
) -> Vec<FullTrack> {
    // accumulate the tracks that are in the source playlist but not in the target playlist
    let mut unsynced_tracks: Vec<FullTrack> = Vec::new();

    // go over all the tracks in the source playlist and check if they are in the target playlist
    for source_item in source_playlist {
        if let Some(PlayableItem::Track(source_track)) = source_item.track {
            let already_synced: bool = track_in_playlist(&source_track, &target_playlist);
            if !already_synced {
                unsynced_tracks.push(source_track);
            }
        }
    }

    return unsynced_tracks;
}
