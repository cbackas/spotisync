import threading, os
from http.server import HTTPServer, SimpleHTTPRequestHandler
from utils import log
import spotipy

code = None

class MyRequestHandler(SimpleHTTPRequestHandler):
    def _set_headers(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.end_headers()

    def _html(self, message):
        content = f"<html><body><h1>{message}</h1></body></html>"
        return content.encode("utf8")

    def do_GET(self):
        self._set_headers()
        if self.path.startswith('/callback?code='):
            log('Callback recieved')
            self.wfile.write(self._html("Thanks for the callback. You can close this tab now."))
            # update code global var so we can get access token using it once server is killed
            global code
            code = self.path.replace('/callback?code=', '')
            # assassinate the server from a new thread
            threading.Thread(target=self.server.shutdown, daemon=True).start()
        else:
            self.wfile.write(self._html("Hi there! How about giving me a callback?"))

def authenticate():
    username = 'thezacgibson'
    scopes = 'playlist-modify-public playlist-modify-private playlist-read-private'
    cache_path = os.path.join('/config', '.auth-cache')
    auth_manager = spotipy.SpotifyOAuth(scope=scopes, cache_path=cache_path, username=username, open_browser=False)

    # Display sign in link when no token
    if not auth_manager.get_cached_token():
        auth_url = auth_manager.get_authorize_url()
        print(f'Sign in with web browser: {auth_url}')

        # spin up a BLOCKING web server that uses MyRequestHandler to wait for a HTTP request to /callback
        server = HTTPServer(('0.0.0.0', 8100), MyRequestHandler)
        log('HTTP server listening for requests')
        server.serve_forever()

        if code != None:
            access_token = auth_manager.get_access_token(code=code, as_dict=False)
            return spotipy.Spotify(auth=access_token)
    else:
        log('Authenticated.')
        return spotipy.Spotify(auth_manager=auth_manager)
    return None