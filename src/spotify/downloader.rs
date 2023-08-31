use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tracing::{debug, error};

pub async fn spotify_bulk_download() {
    debug!("Starting bulk download of Spotify URLs");

    let mut command = Command::new("zspotify");

    command.arg("--download-dir");
    command.arg("/downloads/");
    command.arg("--music-dir");
    command.arg("/music/");
    command.arg("--episodes-dir");
    command.arg("/episodes/");

    command.arg("--audio-format");
    command.arg("mp3");

    command.arg("--skip-downloaded");

    command.arg("--bulk-download");
    command.arg("/.zspotify/bulk_download.txt");

    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    match command.spawn() {
        Ok(mut child) => {
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let stderr = child.stderr.take().expect("Failed to capture stderr");

            let mut stdout_reader = BufReader::new(stdout).lines();
            let mut stderr_reader = BufReader::new(stderr).lines();

            tokio::select! {
                _ = async {
                    while let Some(line) = stdout_reader.next_line().await.unwrap() {
                        let cleaned_line = line.replace("\x1B[2J\x1B[1;1H", ""); // Remove the escape codes for clearing the screen
                        debug!("{}", cleaned_line);
                    }
                } => {},
                _ = async {
                    while let Some(line) = stderr_reader.next_line().await.unwrap() {
                        let cleaned_line = line.replace("\x1B[2J\x1B[1;1H", ""); // Remove the escape codes for clearing the screen
                        error!("{}", cleaned_line);
                    }
                } => {},
            }

            let status = child.wait().await.expect("Failed to wait on child");

            if status.success() {
                debug!("Bulk download finished");
            } else {
                error!("Bulk download failed");
            }
        }
        Err(error) => {
            error!("Failed to execute command: {}", error);
        }
    }
}
