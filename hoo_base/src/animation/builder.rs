use std::time::Duration;
use std::collections::HashMap;

use crate::{HueValue, SaturationValue, BrightnessValue};
use crate::animation::AnimationFrame;

use hoo_api::light::{LightNumber, LightState};
use hoo_api::color::Color;

pub struct AnimationBuilder {
    frames: Vec<AnimationFrame>,
}

impl Default for AnimationBuilder {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
        }
    }
}

impl AnimationBuilder {
    pub fn build(self) -> impl Iterator<Item = AnimationFrame> {
        self.frames.into_iter()
    }

    pub fn with_frame(mut self, frame: AnimationFrame) -> Self {
        self.frames.push(frame);
        self
    }
}

pub struct AnimationFrameBuilder {
    hold_time: Option<Duration>,
    transition_time: Option<Duration>,
    states: HashMap<LightNumber, LightState>,
}

impl Default for AnimationFrameBuilder {
    fn default() -> Self {
        AnimationFrameBuilder {
            hold_time: None,
            transition_time: None,
            states: HashMap::new(),
        }
    }
}

impl AnimationFrameBuilder {
    pub fn with_hold_time(mut self, hold_time: Duration) -> Self {
        self.hold_time = Some(hold_time);
        self
    }

    pub fn with_transition_time(mut self, transition_time: Duration) -> Self {
        self.transition_time = Some(transition_time);
        self
    }

    pub fn with_light_state(mut self, light_number: LightNumber, light_state: LightState) -> Self  {
        self.states.insert(light_number, light_state);
        self
    }

    pub fn with_color_change(mut self, light_numer: LightNumber, color_change: ColorChange) -> Self {
        // I think I actuailly need to encode this into AnimationFrame so that they can use the current
        // state of the light even if it's set from the Hue app
        self
    }

    pub fn build(self) -> AnimationFrame {
        AnimationFrame {
            hold_time: self.hold_time.unwrap_or(Duration::from_secs(0)),
            transition_time: self.transition_time,
            states: self.states.into_iter().collect(),
        }
    }
}

pub enum ColorChange {
    HSB(Color),
    Diff(HueValue, SaturationValue, BrightnessValue),
    RandomHue,
}