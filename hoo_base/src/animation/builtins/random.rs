use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Duration;

use crate::animation::AnimationFrame;

use hoo_api::light::{LightNumber, LightState};
use hoo_api::ApiConnection;

pub struct RandomAnimation {
    transition_time: Duration,
    hold_time: Duration,
    lights: Vec<u8>,
    rng: ThreadRng,
}

impl RandomAnimation {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        connection: &ApiConnection,
        transition_time: &Duration,
        hold_time: &Duration,
    ) -> Result<Self, failure::Error> {
        let lights = connection.get_active_lights()?.0.keys().cloned().collect();

        let anim = Self {
            transition_time: *transition_time,
            hold_time: *hold_time,
            lights,
            rng: rand::thread_rng(),
        };

        Ok(anim)
    }
}

impl Iterator for RandomAnimation {
    type Item = AnimationFrame;

    fn next(&mut self) -> Option<Self::Item> {
        let transition_millis =
            self.transition_time.as_secs() * 1000 + u64::from(self.transition_time.subsec_millis());

        let transition_value = transition_millis as u16 / 100;

        let mut states: Vec<(LightNumber, LightState)> = Vec::new();

        for light_num in &self.lights {
            let next_hue: u16 = self.rng.gen();
            let state = LightState::new()
                .hue(next_hue)
                .sat(255)
                .transitiontime(transition_value);
            states.push((*light_num, state));
        }

        let frame = AnimationFrame {
            transition_time: Some(self.transition_time),
            hold_time: self.hold_time,
            states,
        };

        Some(frame)
    }
}
