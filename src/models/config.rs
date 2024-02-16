use std::{fs, path::PathBuf};

use serde::Deserialize;
use teloxide::types::ChatId;

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub token: String,
    pub ngrok_api_url: String,
    pub chat_id: ChatId,
    pub command_secret: String,
}

impl BotConfig {
    pub fn from_file(url: PathBuf) -> Result<Self, String> {
        let content = read_file_as_string(url);
        match content {
            Ok(content) => {
                let config =
                    serde_yaml::from_str::<BotConfig>(&content).map_err(|e| e.to_string())?;
                Ok(config)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct NgrokConfig {
    pub api_key: String,
    pub url: Option<String>,
}

impl NgrokConfig {
    pub fn from_file(url: PathBuf, api_url: String) -> Result<Self, String> {
        let content = read_file_as_string(url);
        match content {
            Ok(content) => {
                let mut config =
                    serde_yaml::from_str::<NgrokConfig>(&content).map_err(|e| e.to_string())?;
                config.url = Some(api_url);
                Ok(config)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

fn read_file_as_string(file_path: PathBuf) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(file_path)?)
}
