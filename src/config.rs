use std::env;
use std::path::PathBuf;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct U2CliConfig {
    pub nexusphp_u2: Option<String>,
    pub save_dir: String,
}

impl Default for U2CliConfig {
    fn default() -> Self {
        Self {
            nexusphp_u2: None,
            save_dir: default_save_dir(),
        }
    }
}

impl U2CliConfig {
    pub async fn read_or_create() -> anyhow::Result<Self> {
        create_config_dir().await?;
        let config_dir = config_dir()?;
        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            let config = U2CliConfig::default();
            let config_str = toml::to_string(&config)?;
            tokio::fs::write(&config_file, config_str).await?;
        }
        let config_str = tokio::fs::read_to_string(&config_file).await?;
        let config = toml::from_str(&config_str)?;
        Ok(config)
    }
    pub async fn write(&self) -> anyhow::Result<()> {
        let config_dir = config_dir()?;
        let config_file = config_dir.join("config.toml");
        let config_str = toml::to_string(self)?;
        tokio::fs::write(config_file, config_str).await?;
        Ok(())
    }
}

fn default_save_dir() -> String {
    let dir = config_dir().unwrap();
    dir.to_str().unwrap().to_owned()
}

fn config_dir() -> anyhow::Result<PathBuf> {
    let home_dir = env::var("HOME")
        .map_err(|_| anyhow!("Only support linux") )?;
    let dir = PathBuf::from(home_dir).join(".u2");
    Ok(dir)
}

async fn create_config_dir() -> anyhow::Result<()> {
    let dir = config_dir()?;
    if !dir.exists() {
        tokio::fs::create_dir_all(dir).await?;
    }
    Ok(())
}