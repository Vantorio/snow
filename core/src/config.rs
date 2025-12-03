use serde::{Deserialize, Serialize};
use std::{fs, io};
use tracing::info;

const CONFIG_FILE_NAME: &str = "config.toml";

const DEFAULT_NETWORK_HOST: &str = "0.0.0.0";
const DEFAULT_NETWORK_PORT: u16 = 19132;
const DEFAULT_SERVER_NAME: &str = "Snow Server";
const DEFAULT_MAX_PLAYERS: u32 = 20;

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub name: String,
    pub max_players: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub server: ServerConfig,
}

impl Config {
    fn default() -> Self {
        Config {
            network: NetworkConfig {
                host: DEFAULT_NETWORK_HOST.to_string(),
                port: DEFAULT_NETWORK_PORT,
            },
            server: ServerConfig {
                name: DEFAULT_SERVER_NAME.to_string(),
                max_players: DEFAULT_MAX_PLAYERS,
            },
        }
    }

    pub fn new() -> Result<Self, io::Error> {
        if fs::metadata(CONFIG_FILE_NAME).is_err() {
            info!("Configuration file '{}' not found. Creating default file.", CONFIG_FILE_NAME);

            let default_config = Self::default();

            let toml_string = match toml::to_string_pretty(&default_config) {
                Ok(s) => s,
                Err(e) => {
                    return Err(io::Error::other(format!("Failed to serialize default config: {}", e)));
                }
            };

            fs::write(CONFIG_FILE_NAME, toml_string)?;
        }

        let s = match config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse config file: {}", e),
                ));
            }
        };

        match s.try_deserialize() {
            Ok(conf) => Ok(conf),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to deserialize config structure: {}", e),
            )),
        }
    }
}
