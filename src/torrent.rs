use crate::config::U2CliConfig;
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::path::Path;
use tokio::process::Command;
use tokio::fs;

fn torrent_url(torrent_id: i64) -> String {
    format!("https://u2.dmhy.org/download.php?id={}", torrent_id)
}

pub async fn download_torrent(torrent_id: i64) -> Result<PathBuf> {
    let config = U2CliConfig::read_or_create().await?;
    let client = reqwest::Client::new();

    // Get the torrent content with cookie
    let response = client
        .get(torrent_url(torrent_id))
        .header(
            "Cookie",
            format!("nexusphp_u2={}", config.nexusphp_u2.ok_or(anyhow!("Need Cookie"))?)
        )
        .send()
        .await?;

    // Use fixed filename format
    let filename = format!("{}.torrent", torrent_id);
    let save_path = PathBuf::from(&config.save_dir).join(filename);

    // Download and save the file
    let bytes = response.bytes().await?;
    tokio::fs::write(&save_path, bytes).await?;

    Ok(save_path)
}

pub async fn add_to_transmission<P: AsRef<Path>>(torrent_file: P) -> Result<()> {
    let output = Command::new("transmission-remote")
        .arg("-a")
        .arg(torrent_file.as_ref())
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to add torrent: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

pub async fn clean_torrents() -> Result<usize> {
    let config = U2CliConfig::read_or_create().await?;
    let save_dir = PathBuf::from(&config.save_dir);

    // Make sure the directory exists
    if !save_dir.exists() {
        return Ok(0); // No files to clean
    }

    let mut count = 0;
    let mut entries = fs::read_dir(&save_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "torrent" {
                fs::remove_file(&path).await?;
                count += 1;
            }
        }
    }

    Ok(count)
}

