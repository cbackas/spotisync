# spotisync

Simple little script to make sure any song I add to a certain spotify playlist also gets added to another spotify playlist. This takes the place of an IFTT automation that I had running for a while. Gives me faster syncing, less rate limiting, less companies with API access to my accounts, and an excuse to code. 


### Deployed using docker: 
Has port passed through so it can briefly spin up a web server for oauth and volume for giving cached token persistant storage

docker run -d -p 8100:8100 -v /config:/local/path/to/token/cache -e "SPOTIPY_CLIENT_ID=client_id" -e "SPOTIPY_CLIENT_SECRET=client_secret" -e "SPOTIPY_REDIRECT_URI=redirect_uri" cbackas/spotisync:latest
