FROM rust as build
ADD ./src /app/src
ADD ./Cargo.lock /app/Cargo.lock
ADD ./Cargo.toml /app/Cargo.toml
WORKDIR /app
RUN cargo build --release

FROM rust as runtime
COPY --from=build /app/target/release/spotisync /usr/local/bin/spotisync

RUN apt-get update && apt-get install -y python3-pip ffmpeg
RUN pip install --break-system-package git+https://github.com/jsavargas/zspotify

RUN mkdir -p /app/cache
ENV RSPOTIFY_CACHE_PATH="/app/cache/.spotify_token_cache.json"
ENV CALLBACK_HOST="localhost"
ENV CALLBACK_PORT="8100"

USER 99:100

ENTRYPOINT ["spotisync"]
