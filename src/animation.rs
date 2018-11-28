use std::time::Duration;
use std::thread::sleep;

use crate::AnyError;
use crate::api::{ApiConnection, set_state, get_all_lights};
use crate::light::{LightNumber, LightState};

#[derive(Debug, Clone)]
pub struct AnimationFrame {
    pub hold_time: Duration,
    pub transition_time: Duration,
    pub states: Vec<(LightNumber, LightState)>,
}

#[derive(Debug, Clone)]
pub struct Animation {
    frames: Vec<AnimationFrame>,
}

impl Animation {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }

    pub fn with_frame(mut self, frame: AnimationFrame) -> Self {
        self.frames.push(frame);
        self
    }

    pub fn with_frames<I>(mut self, frames: I) -> Self
        where I: IntoIterator<Item = AnimationFrame>
    {
        for frame in frames {
            self.frames.push(frame);
        }
        self
    }

    pub fn play(&self, connection: &ApiConnection) -> Result<(), AnyError> {
        loop {
            for frame in &self.frames {
                let time = (frame.transition_time.as_secs() * 10) + (frame.transition_time.subsec_millis() / 100) as u64;

                for (light_num, state) in &frame.states {
                    if let Some(color) = state.get_color() {
                        println!("{} Next color: {}", light_num, color);
                    }
                    let state = state.clone().transitiontime(time as u16);
                    set_state(connection, *light_num, &state)?;
                }

                println!("Transitioning: {:?}", frame.transition_time);
                sleep(frame.transition_time);

                println!("Holding: {:?}", frame.hold_time);
                sleep(frame.hold_time);
            }
        }
    }
}

pub fn from_current(connection: &ApiConnection, transition_time: &Duration, hold_time: &Duration) -> Result<Animation, AnyError> {
    let all_lights = get_all_lights(connection)?.0;

    let mut active_lights = Vec::new();
    let mut light_states = Vec::new();

    for (light_num, light) in all_lights {
        if !light.state.is_on() || !light.state.is_reachable() {
            continue;
        }

        if let Some(color) = light.state.get_color() {
            active_lights.push(light_num);
            light_states.push(LightState::new().color(&color));
        }
    }

    let mut frames = Vec::new();

    let num_lights = light_states.len();

    for _ in 0 .. num_lights {
        light_states.rotate_right(1);

        let active_lights_copy = active_lights.clone();
        let mut light_states_copy = light_states.clone();

        
        let frame = AnimationFrame {
            hold_time: hold_time.clone(),
            transition_time: transition_time.clone(),
            states: active_lights_copy.into_iter().zip(light_states_copy).collect(),
        };

        frames.push(frame);
    }

    Ok(Animation::new().with_frames(frames))

}