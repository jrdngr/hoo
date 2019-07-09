use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub const DEFAULT_CONFIG_FILE_NAME: &str = "hoo_config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooConfig {
    pub hue_hub_uri: String,
    pub hue_user_id: String,
    pub hoo_server_socket_uri: String,
}

impl Default for HooConfig {
    fn default() -> Self {
        Self {
            hue_hub_uri: "http://<Hue Hub IP>".to_string(),
            hue_user_id: "<40-character Hue User Id>".to_string(),
            hoo_server_socket_uri: "localhost:8000".to_string(),
        }
    }
}

impl HooConfig {
    pub fn from_env() -> Self {
        let hue_hub_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
        let hue_user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");
        let hoo_server_socket_uri = std::env::var("SOCKET_IP").expect("SOCKET_IP must be set");

        Self {
            hue_hub_uri,
            hue_user_id,
            hoo_server_socket_uri,
        }
    }

    pub fn from_dotenv() -> Self {
        dotenv::dotenv().ok();
        Self::from_env()
    }

    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, Box<Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let config = ron::de::from_reader(reader)?;

        Ok(config)
    }

    pub fn from_default_file() -> Result<Self, Box<Error>> {
        Self::from_file(DEFAULT_CONFIG_FILE_NAME)
    }

    pub fn write_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<Error>> {
        let mut file = File::create(file_path)?;
        let contents = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn write_default_file(&self) -> Result<(), Box<Error>> {
        self.write_file(DEFAULT_CONFIG_FILE_NAME)
    }
}
