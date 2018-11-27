use std::time::Duration;
use std::thread::sleep;

use crate::AnyError;
use crate::api::{ApiConnection, set_state};
use crate::light::LightState;

pub struct AnimationFrame {
    pub light_number: u8,
    pub hold_time: Duration,
    pub transition_time: Duration,
    pub state: LightState,
}

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
                if let Some(color) = frame.state.get_color() {
                    println!("Next color: {}", color);
                }

                let time = (frame.transition_time.as_secs() * 10) + (frame.transition_time.subsec_millis() / 100) as u64;
                let state = &frame.state.clone().transitiontime(time as u16);

                set_state(connection, frame.light_number, &state)?;

                println!("Transitioning: {:?}", frame.transition_time);
                sleep(frame.transition_time);

                println!("Holding: {:?}", frame.hold_time);
                sleep(frame.hold_time);
            }
        }
    }
}