use serde::{Deserialize, Serialize};

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
