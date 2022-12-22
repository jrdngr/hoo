use std::collections::HashMap;

use crate::Motion;
use crate::connection::ApiConnection;
use crate::light::{Light, LightCollection, LightEffect, LightState};

pub struct StandardApiConnection {
    pub client: reqwest::Client,
    base_uri: String,
}

impl StandardApiConnection {
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

impl ApiConnection for StandardApiConnection {
    fn get_all_lights(&self) -> Result<LightCollection, failure::Error> {
        let uri = format!("{}/lights", self.base());

        let response = self.client.get(&uri).send()?.text()?;

        let lights = serde_json::from_str(&response)?;

        Ok(lights)
    }

    fn get_active_lights(&self) -> Result<LightCollection, failure::Error> {
        let active_lights: HashMap<u8, Light> = self
            .get_all_lights()?
            .0
            .into_iter()
            .filter(|(_, l)| l.state.is_on() && l.state.is_reachable())
            .collect();

        Ok(LightCollection(active_lights))
    }

    fn get_light(&self, light_number: u8) -> Result<Light, failure::Error> {
        let uri = format!("{}/lights/{}", self.base(), light_number);
        let response = self.client.get(&uri).send()?.text()?;

        let light = serde_json::from_str(&response)?;

        Ok(light)
    }

    fn get_all_motion_sensors(&self) -> Result<Vec<Motion>, failure::Error> {
        let uri = format!("{}/sensors", self.base());

        let response = self.client.get(&uri).send()?.text()?;

        let sensors: HashMap<u32, serde_json::Value> = serde_json::from_str(&response)?;
        
        let mut motion_sensors = Vec::new();
        for (_, sensor) in sensors {
            if let Ok(motion_sensor) = serde_json::from_value(sensor) {
                motion_sensors.push(motion_sensor);
            }
        }

        Ok(motion_sensors)
    }

    fn set_state(&self, light_number: u8, state: &LightState) -> Result<String, failure::Error> {
        let body = serde_json::to_string(state)?;

        let uri = format!("{}/lights/{}/state", self.base(), light_number);

        let response = self.client.put(&uri).body(body).send()?.text()?;

        Ok(response)
    }

    fn on(&self, light_number: u8) -> Result<String, failure::Error> {
        let state = LightState::new().on(true);
        self.set_state(light_number, &state)
    }

    fn off(&self, light_number: u8) -> Result<String, failure::Error> {
        let state = LightState::new().on(false);
        self.set_state(light_number, &state)
    }

    fn colorloop(&self, light_number: u8, enabled: bool) -> Result<String, failure::Error> {
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
    ) -> Result<String, failure::Error> {
        let state = LightState::new().transitiontime(transition_time);
        self.set_state(light_number, &state)
    }
}
