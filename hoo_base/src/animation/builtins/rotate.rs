use std::collections::HashMap;
use std::time::Duration;

use crate::animation::{AnimationFrame, LoopingAnimation};
use hoo_api::light::{Light, LightNumber, LightState};
use hoo_api::ApiConnection;

pub struct RotateAnimation {
    animation: LoopingAnimation,
}

impl RotateAnimation {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        connection: &dyn ApiConnection,
        transition_time: Duration,
        hold_time: Duration,
        light_numbers: &[LightNumber],
        hues: Option<Vec<u16>>,
    ) -> Result<Self, failure::Error> {
        let all_lights = connection.get_active_lights()?.0;

        let selected_lights: HashMap<LightNumber, Light> = all_lights.into_iter()
            .filter(|(light_num, _)| light_numbers.contains(light_num))
            .collect();

        let mut active_lights = Vec::new();
        let mut light_states = Vec::new();

        for (light_num, _light) in &selected_lights {
            active_lights.push(*light_num);
        }

        match hues {
            Some(hues) => {
                let brightness_sum: usize = selected_lights
                    .iter()
                    .flat_map(|(_, light)| light.state.bri)
                    .map(|bri| bri as usize)
                    .sum();

                let average_brightness = brightness_sum / selected_lights.len();

                for hue in hues {
                    light_states.push(
                        LightState::new().hue(hue).sat(255).bri(average_brightness as u8)
                    )
                }
            },
            None => {
                for (_light_num, light) in selected_lights {
                    if let Some(color) = light.state.get_color() {
                        light_states.push(LightState::new().color(&color));
                    }
                }
            },
        }

        let mut frames = Vec::new();

        let num_lights = light_states.len();

        for _ in 0..num_lights {
            light_states.rotate_right(1);

            let active_lights_copy = active_lights.clone();
            let light_states_copy = light_states.clone();

            let frame = AnimationFrame {
                hold_time: hold_time,
                transition_time: Some(transition_time),
                states: active_lights_copy
                    .into_iter()
                    .zip(light_states_copy)
                    .collect(),
            };

            frames.push(frame);
        }

        let animation = Self {
            animation: LoopingAnimation::new().with_frames(frames),
        };

        Ok(animation)
    }
}

impl Iterator for RotateAnimation {
    type Item = AnimationFrame;

    fn next(&mut self) -> Option<Self::Item> {
        self.animation.next()
    }
}
