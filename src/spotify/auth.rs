use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::sync::Mutex;

// gets a spotify client so we can make spotify requests
// first tries to read a cached token
// if it cant do that then it will present a URL for OAuth flow
pub async fn get_spotify_client() -> AuthCodeSpotify {
    let cache_path: String = match env::var_os("RSPOTIFY_CACHE_PATH") {
        Some(v) => v.into_string().unwrap(),
        None => "./cache/.spoitify_token_cache.json".to_string(),
    };

    let config = Config {
        cache_path: PathBuf::from(cache_path),
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    let creds: Credentials = Credentials::from_env()
        .expect("Missing RSPOTIFY_CLIENT_ID RSPOTIFY_CLIENT_SECRET env vars");

    let callback_host: String = match env::var_os("CALLBACK_HOST") {
        Some(v) => v.into_string().unwrap(),
        None => "localhost".to_string(),
    };

    let callback_port: String = match env::var_os("CALLBACK_PORT") {
        Some(v) => v.into_string().unwrap(),
        None => "8100".to_string(),
    };

    let oauth = OAuth {
        redirect_uri: format!("http://{}:{}/callback", callback_host, callback_port),
        scopes: scopes!(
            "playlist-modify-public",
            "playlist-modify-private",
            "playlist-read-private"
        ),
        ..Default::default()
    };

    let spotify: AuthCodeSpotify = AuthCodeSpotify::with_config(creds.clone(), oauth, config);

    let url: String = spotify
        .get_authorize_url(false)
        .expect("Failed to get authorize url");

    // check if theres a cached token
    let existing_token = spotify.read_token_cache(true).await;
    match existing_token {
        // if there is a token then lets use it in our spotify client
        Ok(Some(token)) => {
            *spotify
                .get_token()
                .lock()
                .await
                .expect("Failed to unlock some kinda thing") = Some(token);
            spotify
                .refetch_token()
                .await
                .expect("Failed to refetch token");
        }

        // otherwise we need to get a new token
        _ => {
            // prompt user with url for oauth flow
            println!("Open this url in your browser:\n{}", url);
            let url: String = listen_for_callback().await;
            let code: String = spotify
                .parse_response_code(&url)
                .expect("Failed to parse code from callback url");

            // complete the oauth flow and save the token to the cache
            spotify
                .request_token(&code)
                .await
                .expect("couldn't authenticate successfully");
        }
    }

    return spotify;
}

// Channel used to send shutdown signal - wrapped in an Option to allow
// it to be taken by value (since oneshot channels consume themselves on
// send) and an Arc<Mutex> to allow it to be safely shared between threads
lazy_static! {
    static ref SHUTDOWN_TX: Arc<Mutex<Option<Sender<()>>>> = <_>::default();
    static ref QUERY_TX: Arc<Mutex<Option<Sender<String>>>> = <_>::default();
}

// starts up an http server and waits for a callback from spotify
// returns the entire callback url so we can parse the code/state
async fn listen_for_callback() -> String {
    let callback_port: u16 = match env::var_os("CALLBACK_PORT") {
        Some(v) => v.into_string().unwrap(),
        None => "8100".to_string(),
    }
    .parse()
    .expect("Failed to parse CALLBACK_PORT as u16");

    // define a channel to send the shutdown signal to the main thread
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    SHUTDOWN_TX.lock().await.replace(shutdown_tx);

    // define a channel to send the query string to the main thread
    let (query_tx, query_rx) = tokio::sync::oneshot::channel::<String>();
    QUERY_TX.lock().await.replace(query_tx);

    // create http server
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hande_requests)) });
    let addr: SocketAddr = ([0, 0, 0, 0], callback_port).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Waiting for Spotify OAuth callback...");

    // set the server to shutdown when the shutdown signal is recieved
    let graceful = server.with_graceful_shutdown(async {
        shutdown_rx.await.ok();
    });

    // wait for the server to shutdown before the function will return
    graceful
        .await
        .expect("The callback server didn't shutdown gracefully");

    // wait for the callback request to be recieved, expecting the entire URL
    let callback_url: String = query_rx
        .await
        .expect("Failed to get callback url from query");

    // add the host and port to the callback url
    // kinda stupid but it lets me use AuthCodeSpotify::parse_response_code()
    let callback_url: String = format!("localhost:{}{}", callback_port.to_string(), callback_url);

    return callback_url;
}

// handle all http requests
async fn hande_requests(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // if callback url send the shutdown signal and the code/state signal
    if req.uri().path() == "/callback" {
        // call the shutdown signal
        if let Some(tx) = SHUTDOWN_TX.lock().await.take() {
            let _ = tx.send(());
        }

        // signal the callback code back to the main thread
        if let Some(tx) = QUERY_TX.lock().await.take() {
            let _ = tx.send(req.uri().to_string());
        }

        return Ok(Response::new(Body::from(
            "Thanks for authenticating Spotisync",
        )));
    }

    Ok(Response::new(Body::from(
        "That's not a Spotify callback URL",
    )))
}
