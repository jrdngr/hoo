use serde_derive::Deserialize;

pub struct ApiConnection {
    pub client: reqwest::Client,
    base_uri: String,
    user_id: String,
}

impl ApiConnection {
    pub fn new(base_uri: &str, user_id: &str) -> Self {
        Self { 
            client: reqwest::Client::new(),
            base_uri: base_uri.to_string(),
            user_id: user_id.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Light {
    pub name: String,
    pub state: LightState,
}

#[derive(Debug, Deserialize)]
pub struct LightState {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8,
    pub xy: (f32, f32),
    pub ct: u16,
    pub effect: LightEffect,
    pub alert: LightAlert,
    pub transitiontime: u16,
    pub bri_inc: i16,
    pub sat_inc: i16,
    pub hue_inc: i32,
    pub ct_inc: i32,
    pub xy_inc: (f32, f32),
    pub colormode: LightColorMode,
    pub reachable: bool,
}

#[derive(Debug, Deserialize)]
pub enum LightEffect {
    None,
    ColorLoop,
}

#[derive(Debug, Deserialize)]
pub enum LightAlert {
    None,
    Select,
    Lselect,
}

#[derive(Debug, Deserialize)]
pub enum LightColorMode {
    HS,
    XY,
    CT,
}