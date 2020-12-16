import time, datetime
from spotipy.client import Spotify

# have to deal with paginated track lists cuz playlists be big
# https://stackoverflow.com/questions/39086287/spotipy-how-to-read-more-than-100-tracks-from-a-playlist
def get_playlist_tracks(sp: Spotify, playlist_id: str):
    results = sp.playlist_tracks(playlist_id)
    tracks = results['items']
    while results['next']:
        results = sp.next(results)
        tracks.extend(results['items'])
    return tracks

# sync the playlists
def perform_sync(sp: Spotify):
    current_time = datetime.datetime.fromtimestamp(time.time()).strftime('%Y-%m-%d %H:%M:%S')
    print(f'[{current_time}] Checking playlists for differences...')

    items_jams = get_playlist_tracks(sp, '3KAGyeFZK1uDfet9hOd6gU')
    tracks_jams = [item['track']['id'] for item in items_jams]
    items_jelly = get_playlist_tracks(sp, '6cHhVGOS9UBamBzw53SQZL')
    unsynced_tracks = [item['track']['id'] for item in items_jelly if item['track']['id'] not in tracks_jams]

    # add jelly songs not in jams to jams
    if len(unsynced_tracks) >= 1:
        print(f'     Syncing tracks: {unsynced_tracks}')
        sp.playlist_add_items(playlist_id='3KAGyeFZK1uDfet9hOd6gU', items=unsynced_tracks)