use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use failure::format_err;

use crate::Motion;
use crate::connection::ApiConnection;
use crate::light::{Light, LightCollection, LightEffect, LightState};

pub struct TestingApiConnection {
    file_path: PathBuf,
}

impl TestingApiConnection {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, failure::Error> {
        let mut path = PathBuf::new();
        path.push(file_path);

        if path.exists() {
            Ok(Self { file_path: path })
        } else {
            let light_state = LightState {
                on: Some(true),
                bri: Some(127),
                hue: Some(0),
                sat: Some(255),
                ..Default::default()
            };

            let light = Light {
                name: "Mr. Light".to_string(),
                state: light_state,
            };

            let mut collection_map = HashMap::new();
            collection_map.insert(1, light);

            let collection = LightCollection(collection_map);
            let connection = Self { file_path: path };

            connection.set_local_state(collection)?;

            Ok(connection)
        }
    }

    pub fn get_local_state(&self) -> Result<LightCollection, failure::Error> {
        if !self.file_path.exists() {
            return Ok(LightCollection(HashMap::new()));
        }

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let collection = ron::de::from_reader(reader)?;

        Ok(collection)
    }

    pub fn set_local_state(&self, lights: LightCollection) -> Result<(), failure::Error> {
        let mut file = File::create(&self.file_path)?;
        let contents = ron::ser::to_string_pretty(&lights, ron::ser::PrettyConfig::default())?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

impl ApiConnection for TestingApiConnection {
    fn get_all_lights(&self) -> Result<LightCollection, failure::Error> {
        let lights = self.get_local_state()?;
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
        let all_lights = self.get_all_lights()?.0;
        let light = all_lights
            .get(&light_number)
            .ok_or_else(|| format_err!("Light {} not found", &light_number))?;

        Ok(light.clone())
    }

    fn get_all_motion_sensors(&self) -> Result<Vec<Motion>, failure::Error> {
        unimplemented!()
    }

    fn set_state(&self, light_number: u8, state: &LightState) -> Result<String, failure::Error> {
        let mut light = self.get_light(light_number)?;
        light.state = LightState::combine(&light.state, state);

        let mut all_lights = self.get_local_state()?;
        all_lights.0.insert(light_number, light);
        self.set_local_state(all_lights)?;

        Ok(format!("Light number {} state set", &light_number))
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
