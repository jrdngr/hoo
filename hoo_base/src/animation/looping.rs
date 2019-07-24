use std::time::Duration;

use crate::animation::AnimationFrame;

#[derive(Debug, Default, Clone)]
pub struct LoopingAnimation {
    frames: Vec<AnimationFrame>,
    current_index: usize,
}

impl LoopingAnimation {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            current_index: 0,
        }
    }

    pub fn with_frame(mut self, frame: AnimationFrame) -> Self {
        self.frames.push(frame);
        self
    }

    pub fn with_frames<I>(mut self, frames: I) -> Self
    where
        I: IntoIterator<Item = AnimationFrame>,
    {
        for frame in frames {
            self.frames.push(frame);
        }
        self
    }
}

impl Iterator for LoopingAnimation {
    type Item = AnimationFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames.is_empty() {
            return None;
        }

        if self.current_index >= self.frames.len() {
            self.current_index = 0;
        }

        let mut next_frame = self.frames[self.current_index].clone();
        self.current_index += 1;

        let temporary_transition_time =
            next_frame.transition_time.unwrap_or(Duration::from_secs(0));

        let transition_millis = temporary_transition_time.as_secs() * 1000
            + u64::from(temporary_transition_time.subsec_millis());
        let transition_value = transition_millis as u16 / 100;

        for state in next_frame.states.values_mut() {
            state.transitiontime = Some(transition_value);
        }

        Some(next_frame)
    }
}
