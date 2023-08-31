# FROM rust as build_downonspot
# RUN apt-get update && apt-get install -y libasound2-dev libmp3lame-dev
#
# ADD ./DownOnSpot/src /app/src
# ADD ./DownOnSpot/Cargo.lock /app/Cargo.lock
# ADD ./DownOnSpot/Cargo.toml /app/Cargo.toml
# ADD ./DownOnSpot/build.rs /app/build.rs
#
# WORKDIR /app
# RUN cargo build --release

# RUN apt-get update && apt-get install -y libasound2-dev libmp3lame-dev
#
# COPY --from=build_downonspot /app/target/release/down_on_spot /usr/local/bin/down_on_spot

FROM rust as build
ADD ./src /app/src
ADD ./Cargo.lock /app/Cargo.lock
ADD ./Cargo.toml /app/Cargo.toml
WORKDIR /app
RUN cargo build --release

FROM rust as runtime
COPY --from=build /app/target/release/spotisync /usr/local/bin/spotisync

RUN apt-get update && apt-get install -y python3-pip
RUN pip install --break-system-package git+https://github.com/jsavargas/zspotify

RUN mkdir -p /app/cache
ENV RSPOTIFY_CACHE_PATH="/app/cache/.spotify_token_cache.json"
ENV CALLBACK_HOST="localhost"
ENV CALLBACK_PORT="8100"

ENTRYPOINT ["spotisync"]
