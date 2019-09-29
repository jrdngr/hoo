use std::collections::HashMap;
use std::time::Duration;

use hoo_api_types::{LightNumber, LightState};

pub mod builtins;
pub mod dynamic;
pub mod looping;

pub use self::dynamic::DynamicAnimation;
pub use self::looping::LoopingAnimation;

pub type Animation = dyn Iterator<Item = AnimationFrame>;

#[derive(Debug, Default, Clone)]
pub struct AnimationFrame {
    pub hold_time: Duration,
    pub transition_time: Option<Duration>,
    pub states: HashMap<LightNumber, LightState>,
}
