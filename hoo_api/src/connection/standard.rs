use anyhow::Result;

use std::collections::HashMap;

use crate::connection::ApiConnection;
use crate::light::{Light, LightCollection, LightEffect, LightState};

pub struct StandardApiConnection {
    pub client: reqwest::blocking::Client,
    base_uri: String,
}

impl StandardApiConnection {
    pub fn new(base_uri: &str, user_id: &str) -> Self {
        let base_uri = format!("{}/api/{}", base_uri, user_id);
        Self {
            client: reqwest::blocking::Client::new(),
            base_uri: base_uri.to_string(),
        }
    }

    pub fn base(&self) -> String {
        self.base_uri.clone()
    }
}

impl ApiConnection for StandardApiConnection {
    fn get_all_lights(&self) -> Result<LightCollection> {
        let uri = format!("{}/lights", self.base());

        let response = self.client.get(&uri).send()?.text()?;

        let lights = serde_json::from_str(&response)?;

        Ok(lights)
    }

    fn get_active_lights(&self) -> Result<LightCollection> {
        let active_lights: HashMap<u8, Light> = self
            .get_all_lights()?
            .into_iter()
            .filter(|(_, l)| l.state.is_on() && l.state.is_reachable())
            .collect();

        Ok(active_lights)
    }

    fn get_light(&self, light_number: u8) -> Result<Light> {
        let uri = format!("{}/lights/{}", self.base(), light_number);
        let response = self.client.get(&uri).send()?.text()?;

        let light = serde_json::from_str(&response)?;

        Ok(light)
    }

    fn set_state(&self, light_number: u8, state: &LightState) -> Result<String> {
        let body = serde_json::to_string(state)?;

        let uri = format!("{}/lights/{}/state", self.base(), light_number);

        let response = self.client.put(&uri).body(body).send()?.text()?;

        Ok(response)
    }

    fn on(&self, light_number: u8) -> Result<String> {
        let state = LightState::new().on(true);
        self.set_state(light_number, &state)
    }

    fn off(&self, light_number: u8) -> Result<String> {
        let state = LightState::new().on(false);
        self.set_state(light_number, &state)
    }

    fn colorloop(&self, light_number: u8, enabled: bool) -> Result<String> {
        let effect = if enabled {
            LightEffect::ColorLoop
        } else {
            LightEffect::None
        };
        let state = LightState::new().effect(effect);
        self.set_state(light_number, &state)
    }

    fn transition_time(
        &self,
        light_number: u8,
        transition_time: u16,
    ) -> Result<String> {
        let state = LightState::new().transitiontime(transition_time);
        self.set_state(light_number, &state)
    }
}
