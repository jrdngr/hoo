use serde_derive::Deserialize;
use reqwest::Client;

use crate::AnyError;
use crate::light::{Light, LightState, LightEffect};

pub struct ApiConnection {
    pub client: reqwest::Client,
    base_uri: String,
}

impl ApiConnection {
    pub fn new(base_uri: &str, user_id: &str) -> Self {
        let base_uri = format!("{}/api/{}", base_uri, user_id);
        Self { 
            client: reqwest::Client::new(),
            base_uri: base_uri.to_string(),
        }
    }

    pub fn base(&self) -> String {
        self.base_uri.clone()
    }
}

pub fn get_light(connection: &ApiConnection, light_number: u8) -> Result<Light, AnyError> {
    let uri = format!("{}/lights/{}", connection.base(), light_number);

    let response = connection.client.get(&uri)
        .send()?
        .text()?;

    let light = serde_json::from_str(&response)?;

    Ok(light)
}

pub fn set_state(connection: &ApiConnection, light_number: u8, state: &LightState) -> Result<String, AnyError> {
    let body = serde_json::to_string(state)?;

    let uri = format!("{}/lights/{}/state", connection.base(), light_number);

    let response = connection.client.put(&uri)
        .body(body)
        .send()?
        .text()?;

    Ok(response)
}


pub fn on(connection: &ApiConnection, light_number: u8) -> Result<String, AnyError> {
    let state = LightState::new().on(true);
    set_state(connection, light_number, &state)
}

pub fn off(connection: &ApiConnection, light_number: u8) -> Result<String, AnyError> {
    let state = LightState::new().on(false);
    set_state(connection, light_number, &state)
}

pub fn colorloop(connection: &ApiConnection, light_number: u8, enabled: bool) -> Result<String, AnyError> {
    let effect = if enabled { LightEffect::ColorLoop }  else { LightEffect::None };
    let state = LightState::new().effect(effect);
    set_state(connection, light_number, &state)
}

pub fn transition_time(connection: &ApiConnection, light_number: u8, transition_time: u16) -> Result<String, AnyError> {
    let state = LightState::new().transitiontime(transition_time);
    set_state(connection, light_number, &state)
}