FROM rust as build
ADD ./spotisync/ /app/spotisync/
ADD ./spotidownload/ /app/spotidownload/
ADD ./Cargo.lock /app/Cargo.lock
ADD ./Cargo.toml /app/Cargo.toml
WORKDIR /app
RUN cargo build --release

FROM rust as spotisync_runtime
COPY --from=build /app/target/release/spotisync /usr/local/bin/spotisync

RUN mkdir -p /app/cache
ENV RSPOTIFY_CACHE_PATH="/app/cache/.spotify_token_cache.json"
ENV CALLBACK_HOST="localhost"
ENV CALLBACK_PORT="8100"

ENTRYPOINT ["spotisync"]

FROM rust as spotidownload_runtime
COPY --from=build /app/target/release/spotidownload /usr/local/bin/spotidownload

RUN apt-get update && apt-get install -y ffmpeg pipx && pipx ensurepath
RUN pipx install git+https://github.com/jsavargas/zspotify && export PATH=$PATH:/root/.local/bin

ENTRYPOINT ["spotidownload"]
