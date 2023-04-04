use futures_util::TryStreamExt;
use rspotify::{
    model::{FullTrack, PlayableItem, PlaylistId, PlaylistItem},
    prelude::*,
    AuthCodeSpotify,
};

pub async fn one_way_sync(
    spotify: AuthCodeSpotify,
    source_playlist_id: PlaylistId<'_>,
    target_playlist_id: PlaylistId<'_>,
) {
    if source_playlist_id == target_playlist_id {
        panic!("Source and target playlist are the same");
    }

    let source_playlist: Vec<PlaylistItem> = spotify
        .playlist_items(source_playlist_id, None, None)
        .try_collect()
        .await
        .unwrap();

    let target_playlist: Vec<PlaylistItem> = spotify
        .playlist_items(target_playlist_id.as_ref(), None, None)
        .try_collect()
        .await
        .unwrap();

    let unsynced_tracks: Vec<PlayableId> = get_unsynced_tracks(source_playlist, target_playlist)
        .into_iter()
        .map(|track| PlayableId::Track(track.id.unwrap()))
        .collect();

    if unsynced_tracks.is_empty() {
        println!("No tracks to sync");
        return;
    }

    let sync_result = spotify
        .playlist_add_items(target_playlist_id, unsynced_tracks, None)
        .await;

    match sync_result {
        Ok(_) => println!("Successfully synced playlists"),
        Err(e) => println!("Error syncing playlists: {}", e),
    }
}

fn track_in_playlist(track: FullTrack, playlist: Vec<PlaylistItem>) -> bool {
    let contains_track = playlist.into_iter().any(|playlist_item| {
        if let Some(PlayableItem::Track(target_track)) = playlist_item.track.clone() {
            target_track.id.unwrap().id() == track.id.as_ref().unwrap().id()
        } else {
            false
        }
    });

    return contains_track;
}

fn get_unsynced_tracks(
    source_playlist: Vec<PlaylistItem>,
    target_playlist: Vec<PlaylistItem>,
) -> Vec<FullTrack> {
    let mut unsynced_tracks: Vec<FullTrack> = Vec::new();

    for source_item in source_playlist {
        if let Some(PlayableItem::Track(source_track)) = source_item.track {
            let already_synced: bool =
                track_in_playlist(source_track.clone(), target_playlist.clone());
            if !already_synced {
                unsynced_tracks.push(source_track);
            }
        }
    }

    return unsynced_tracks;
}
