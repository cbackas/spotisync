# Spotisync

Simple little script to make sure any song I add to a certain spotify playlist also gets added to another spotify playlist. This takes the place of an IFTT automation that I had running for a while. Gives me faster syncing, less rate limiting, less companies with API access to my accounts, and an excuse to code. 

It was Python, but I've since rewritten it in Rust.


#### Deployed using Docker: 
Has port passed through so it can briefly spin up a web server for oauth and volume for giving cached token persistent storage

> docker run -p 8100:8100 -v /Users/zac/Projects/spotisync/cache:/app/cache -e "RSPOTIFY_CLIENT_ID=client_id" -e "RSPOTIFY_CLIENT_SECRET=client_secret" -e "CALLBACK_HOST=localhost" -e "CALLBACK_PORT=8100" -e "CONTINUOUS_SYNC=true" -e "SYNC_SOURCE_PLAYLIST_ID=playlist_id" -e "SYNC_TARGET_PLAYLIST_ID=playlist_id" spotisync:latest
