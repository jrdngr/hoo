use anyhow::Result;
use std::time::Duration;

use crate::animation::AnimationFrame;

use hoo_api::light::{LightCollection, LightNumber};
use hoo_api::ApiConnection;

pub mod configurable_value;
pub mod producer;
pub mod transform;

pub use configurable_value::ConfigurableValue;
pub use producer::{
    BoxedValueProducer, ConstantProducer, RandomProducer, RandomRangeProducer, ValueProducer,
};
pub use transform::transform;

pub struct DynamicAnimation<'a> {
    connection: &'a ApiConnection,
    hold_time: Duration,
    steps: Vec<DynamicAnimationStep>,
    current_index: usize,
}

impl<'a> DynamicAnimation<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        connection: &'a ApiConnection,
        hold_time: &Duration,
    ) -> Result<Self> {
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

    pub async fn next(&mut self) -> Option<AnimationFrame> {
        self.connection
            .get_active_lights()
            .await
            .ok()
            .map(|lights| self.next_frame(lights))
    }

    pub fn next_frame(&mut self, lights: LightCollection) -> AnimationFrame {
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
    pub operations: Vec<(LightNumber, ConfigurableValue)>,
}

impl DynamicAnimationStep {
    pub fn as_animation_frame(
        &mut self,
        lights: LightCollection,
        hold_time: &Duration,
    ) -> AnimationFrame {
        let states = transform(lights, &mut self.operations);

        AnimationFrame {
            hold_time: *hold_time,
            transition_time: Some(Duration::from_secs(5)),
            states,
        }
    }
}
