from core import current_timestamp
from requests import RequestException
from spotipy.exceptions import SpotifyException

# have to deal with paginated track lists cuz playlists be big
# https://stackoverflow.com/questions/39086287/spotipy-how-to-read-more-than-100-tracks-from-a-playlist
def get_playlist_tracks(sp, playlist_id: str):
    try:
        results = sp.playlist_tracks(playlist_id)
        tracks = results['items']
        while results['next']:
            results = sp.next(results)
            tracks.extend(results['items'])
        return tracks
    except RequestException as e:
        print(f'[{current_timestamp()}] [ERROR] Caught exception while getting playlist tracks: {e}')
        return None
    except SpotifyException as e:
        print(f'[{current_timestamp()}] [ERROR] Caught exception while getting playlist tracks: {e}')
        return None

# sync the playlists
def perform_sync(sp):
    # print(f'[{current_time()}] Checking playlists for differences...')

    items_jam = get_playlist_tracks(sp, '3KAGyeFZK1uDfet9hOd6gU')
    items_jelly = get_playlist_tracks(sp, '6cHhVGOS9UBamBzw53SQZL')
    if items_jam and items_jelly:
        # list of all track IDs in jam
        tracks_jam = [item['track']['id'] for item in items_jam]
        # all tracks that are in jelly but not tracks_jam
        # dict of id:name so i can also print names
        unsynced_tracks = {item['track']['id']:item['track']['name'] for item in items_jelly if item['track']['id'] not in tracks_jam}

        # add jelly songs not in jams to jams
        if len(unsynced_tracks) >= 1:
            try:
                sp.playlist_add_items(playlist_id='3KAGyeFZK1uDfet9hOd6gU', items=list(unsynced_tracks.keys()))

                print(f'[{current_timestamp()}] Synced tracks: {list(unsynced_tracks.values())}')
            except RequestException as e:
                print(f'[{current_timestamp()}] [ERROR] Caught exception while syncing tracks: {e}')
            except SpotifyException as e:
                print(f'[{current_timestamp()}] [ERROR] Caught exception while syncing tracks: {e}')
