use actix_web::{error, http, HttpResponse};
use failure::Fail;
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

#[derive(Debug, Deserialize)]
pub struct AnimationQuery {
    pub lights: String,
    pub hues: Option<String>,
}

impl AnimationQuery {
    pub fn light_vec(&self) -> Vec<u8> {
        use std::str::FromStr;

        self.lights.split(',')
            .map(u8::from_str)
            .filter_map(Result::ok)
            .collect()
    }

    pub fn hue_vec(&self) -> Option<Vec<u16>> {
        match &self.hues {
            Some(hues) => Some({
                use std::str::FromStr;

                hues.split(',')
                    .map(u16::from_str)
                    .filter_map(Result::ok)
                    .collect()
        
            }),
            None => None,
        }
    }
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

#[derive(Fail, Debug)]
#[fail(display = "Internal server error")]
pub struct HooError {}

impl Default for HooError {
    fn default() -> Self {
        HooError {}
    }
}

impl error::ResponseError for HooError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
