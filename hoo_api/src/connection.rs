use anyhow::Result;

use crate::light::{Light, LightCollection, LightState};

pub mod standard;
pub mod testing;

pub use self::standard::StandardApiConnection;
pub use self::testing::TestingApiConnection;

pub trait ApiConnection {
    fn get_all_lights(&self) -> Result<LightCollection>;
    fn get_active_lights(&self) -> Result<LightCollection>;
    fn get_light(&self, light_number: u8) -> Result<Light>;
    fn set_state(&self, light_number: u8, state: &LightState) -> Result<String>;
    fn on(&self, light_number: u8) -> Result<String>;
    fn off(&self, light_number: u8) -> Result<String>;
    fn colorloop(&self, light_number: u8, enabled: bool) -> Result<String>;
    fn transition_time(
        &self,
        light_number: u8,
        transition_time: u16,
    ) -> Result<String>;
}
