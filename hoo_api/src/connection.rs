use hoo_api_types::{Light, LightCollection, LightState};

pub mod standard;
pub mod testing;

pub use self::standard::StandardApiConnection;
pub use self::testing::TestingApiConnection;

pub trait ApiConnection {
    fn get_all_lights(&self) -> Result<LightCollection, failure::Error>;
    fn get_active_lights(&self) -> Result<LightCollection, failure::Error>;
    fn get_light(&self, light_number: u8) -> Result<Light, failure::Error>;
    fn set_state(&self, light_number: u8, state: &LightState) -> Result<String, failure::Error>;
    fn on(&self, light_number: u8) -> Result<String, failure::Error>;
    fn off(&self, light_number: u8) -> Result<String, failure::Error>;
    fn colorloop(&self, light_number: u8, enabled: bool) -> Result<String, failure::Error>;
    fn transition_time(
        &self,
        light_number: u8,
        transition_time: u16,
    ) -> Result<String, failure::Error>;
}
