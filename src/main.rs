mod config;
mod torrent;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;
use crate::config::U2CliConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the CLI
    Config {
        /// Set the nexusphp_u2 cookie
        #[arg(long)]
        cookie: Option<String>,

        /// Set the directory to save torrents
        #[arg(long)]
        save_dir: Option<String>,
    },

    /// Download a torrent and add it to transmission
    Download {
        /// The torrent ID to download
        #[arg(required = true)]
        torrent_id: i64,

        /// Clean (delete) the torrent file after uploading to transmission
        #[arg(short = 'c', long = "clean")]
        clean: bool,
    },

    /// Clean all torrents from the save directory
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Config { cookie, save_dir } => {
            handle_config(cookie, save_dir).await?
        },
        Commands::Download { torrent_id, clean } => {
            handle_download(*torrent_id, *clean).await?
        },
        Commands::Clean => {
            handle_clean().await?
        },
    }

    Ok(())
}

async fn handle_config(cookie: &Option<String>, save_dir: &Option<String>) -> Result<()> {
    let mut config = U2CliConfig::read_or_create().await?;
    let mut updated = false;

    if let Some(cookie_value) = cookie {
        config.nexusphp_u2 = Some(cookie_value.clone());
        updated = true;
        println!("Cookie set successfully");
    }

    if let Some(dir) = save_dir {
        let path = PathBuf::from(dir);
        if !path.exists() {
            tokio::fs::create_dir_all(&path).await?;
        }
        config.save_dir = dir.clone();
        updated = true;
        println!("Save directory set to: {}", dir);
    }

    if updated {
        config.write().await?;
    } else {
        // If no options provided, display current config
        println!("Current configuration:");
        println!("Cookie: {}", config.nexusphp_u2.unwrap_or_else(|| "Not set".to_string()));
        println!("Save directory: {}", config.save_dir);
    }

    Ok(())
}

async fn handle_download(torrent_id: i64, clean: bool) -> Result<()> {
    println!("Downloading torrent {}...", torrent_id);

    let torrent_path = torrent::download_torrent(torrent_id).await?;
    println!("Torrent saved to: {}", torrent_path.display());

    println!("Adding to transmission...");
    torrent::add_to_transmission(&torrent_path).await?;
    println!("Torrent added to transmission successfully");

    if clean {
        println!("Cleaning up torrent file...");
        tokio::fs::remove_file(&torrent_path).await?;
        println!("Torrent file deleted");
    }

    Ok(())
}

async fn handle_clean() -> Result<()> {
    println!("Cleaning torrents from save directory...");

    let count = torrent::clean_torrents().await?;
    println!("Cleaned {} torrent files", count);

    Ok(())
}
