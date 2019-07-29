use std::collections::HashMap;
use std::time::Duration;

use crate::animation::AnimationFrame;

use hoo_api::light::{LightCollection, LightNumber, LightState};
use hoo_api::ApiConnection;

pub mod operation;
pub mod producer;
pub mod transform;

pub use operation::{LightOnStateOperation, LightStateValueFunction, LightStateValueOperation};
pub use producer::{
    ConstantProducer, RandomProducer, RandomRangeProducer, ValueProducer,
};
pub use transform::LightStateTransform;

pub struct DynamicAnimation<'a> {
    connection: &'a ApiConnection,
    hold_time: Duration,
    steps: Vec<DynamicAnimationStep>,
    current_index: usize,
}

impl<'a> Iterator for DynamicAnimation<'a> {
    type Item = AnimationFrame;

    fn next(&mut self) -> Option<Self::Item> {
        match self.connection.get_active_lights() {
            Ok(lights) => Some(self.next_frame(&lights)),
            Err(_) => None,
        }
    }
}

impl<'a> DynamicAnimation<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        connection: &'a ApiConnection,
        hold_time: &Duration,
    ) -> Result<Self, failure::Error> {
        Ok(Self {
            connection,
            hold_time: *hold_time,
            steps: Vec::new(),
            current_index: 0,
        })
    }

    pub fn animation_step(&mut self, step: DynamicAnimationStep) {
        self.steps.push(step);
    }

    pub fn next_frame(&mut self, lights: &LightCollection) -> AnimationFrame {
        if self.steps.is_empty() {
            return Default::default();
        }

        if self.current_index >= self.steps.len() {
            self.current_index = 0;
        }

        let next_step = &mut self.steps[self.current_index];
        self.current_index += 1;

        next_step.as_animation_frame(lights, &self.hold_time)
    }
}

pub struct DynamicAnimationStep {
    pub transforms: HashMap<LightNumber, LightStateTransform>,
}

impl DynamicAnimationStep {
    pub fn as_animation_frame(
        &mut self,
        lights: &LightCollection,
        hold_time: &Duration,
    ) -> AnimationFrame {
        let mut states: HashMap<LightNumber, LightState> = HashMap::new();

        for (light_num, transform) in &mut self.transforms {
            if lights.0.contains_key(&light_num) {
                let new_state = transform.create_light_state(*light_num, lights);
                states.insert(*light_num, new_state);
            }
        }

        AnimationFrame {
            hold_time: *hold_time,
            transition_time: None,
            states,
        }
    }
}
