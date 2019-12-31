use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RGB {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct AnimationSettings {
    pub transition_time: u16,
    pub hold_time: u16,
}

#[derive(Debug, Serialize)]
pub struct HooResponse {
    pub message: String,
}

impl Default for HooResponse {
    fn default() -> Self {
        HooResponse {
            message: "success".to_string(),
        }
    }
}
