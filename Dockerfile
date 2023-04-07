FROM rust as build
ADD ./src /app/src
ADD ./Cargo.lock /app/Cargo.lock
ADD ./Cargo.toml /app/Cargo.toml
WORKDIR /app
RUN cargo build --release

FROM rust as runtime
COPY --from=build /app/target/release/spotisync /usr/local/bin/spotisync
COPY ./entrypoint.sh /usr/local/bin/entrypoint.sh

RUN mkdir -p /app/cache
ENV RSPOTIFY_CACHE_PATH="/app/cache/.spotify_token_cache.json"
ENV CONTINUOUS_SYNC="true"
ENV CALLBACK_HOST="localhost"
ENV CALLBACK_PORT="8100"

CMD ["/usr/local/bin/entrypoint.sh"]
