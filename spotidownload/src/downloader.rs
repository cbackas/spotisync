use std::{
    env,
    fmt::{Debug, Formatter},
};

use anyhow::Error;
use serde::Deserialize;
use tokio::{io::AsyncBufReadExt, io::BufReader, process::Command};
use tokio_stream::{wrappers::LinesStream, StreamExt};
use tracing::{debug, error, trace};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Album,
    Track,
    Artist,
    Playlist,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub r#type: ItemType,
}

pub struct DownloadQueue {
    queue: Vec<Item>,
}

impl DownloadQueue {
    pub fn new() -> Self {
        DownloadQueue { queue: Vec::new() }
    }

    pub fn add(&mut self, item: Item) {
        let exists = self.queue.contains(&item);

        if exists {
            debug!("Item already exists in queue: {:?}", item);
            return;
        }

        self.queue.push(item);
        trace!("Added item to queue: {:?}", self.queue)
    }

    pub fn next(&mut self) -> Option<Item> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}

impl Debug for DownloadQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DownloadQueue")
            .field("queue", &self.queue)
            .finish()
    }
}

struct ZSpotifyFolders {
    download_dir: String,
    music_dir: String,
    episodes_dir: String,
    archive_file: String,
    credentials_file: String,
}

fn get_destinations() -> ZSpotifyFolders {
    let download_dir = env::var("ZSPOTIFY_DOWNLOAD_DIR").unwrap_or("./junk/downloads".to_string());
    let music_dir = env::var("ZSPOTIFY_MUSIC_DIR").unwrap_or("./junk/music".to_string());
    let episodes_dir = env::var("ZSPOTIFY_EPISODES_DIR").unwrap_or("./junk/episodes".to_string());
    let archive_file =
        env::var("ZSPOTIFY_ARCHIVE_FILE").unwrap_or("~/.zspotify/archive.json".to_string());
    let credentials_file =
        env::var("ZSPOTIFY_CREDENTIALS_FILE").unwrap_or("~/.zspotify/credentials.json".to_string());
    ZSpotifyFolders {
        download_dir,
        music_dir,
        episodes_dir,
        archive_file,
        credentials_file,
    }
}

pub async fn download_item(item: Item) -> Result<String, Error> {
    debug!("Starting download of {}", item.name);

    let destinations = get_destinations();

    let mut process = Command::new("zspotify")
        .arg("--download-dir")
        .arg(destinations.download_dir)
        .arg("--music-dir")
        .arg(destinations.music_dir)
        .arg("--episodes-dir")
        .arg(destinations.episodes_dir)
        .arg("--archive")
        .arg(destinations.archive_file)
        .arg("--audio-format")
        .arg("--credentials-file")
        .arg(destinations.credentials_file)
        .arg("mp3")
        .arg("--skip-downloaded")
        .arg(match item.r#type {
            ItemType::Album => "--album",
            ItemType::Track => "--track",
            ItemType::Artist => "--artist",
            ItemType::Playlist => "--playlist",
        })
        .arg(&item.id)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut stdout = LinesStream::new(BufReader::new(process.stdout.take().unwrap()).lines());
    let mut stderr = LinesStream::new(BufReader::new(process.stderr.take().unwrap()).lines());

    tokio::spawn(async move {
        while let Some(msg) = stdout.next().await {
            match msg {
                Ok(line) => debug!("{:?}", line),
                Err(e) => error!("Error reading line: {}", e),
            }
        }
    });

    tokio::spawn(async move {
        while let Some(msg) = stderr.next().await {
            match msg {
                Ok(line) => debug!("{:?}", line),
                Err(e) => error!("Error reading line: {}", e),
            }
        }
    });

    let result = process.wait().await?;
    if !result.success() {
        return Err(Error::msg("Failed to download item"));
    }

    Ok(item.name)
}
