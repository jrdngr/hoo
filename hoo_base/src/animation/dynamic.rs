use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Standard, uniform::SampleUniform};

use std::time::Duration;
use std::collections::HashMap;
use std::ops::{Add, Mul};

use crate::animation::AnimationFrame;

use hoo_api::light::{LightNumber, LightState, LightCollection};
use hoo_api::ApiConnection;

pub struct DynamicAnimation<'a> {
    connection: &'a ApiConnection,
    hold_time: Duration,
    steps: Vec<DynamicAnimationStep>,
    current_index: usize,
}

impl <'a> Iterator for DynamicAnimation<'a> {
    type Item = AnimationFrame;

    fn next(&mut self) -> Option<Self::Item> {
        match self.connection.get_active_lights() {
            Ok(lights) => Some(self.next_frame(&lights)),
            Err(_) => None,
        }
    }
}

impl <'a> DynamicAnimation<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(connection: &'a ApiConnection, hold_time: &Duration) -> Result<Self, failure::Error> {
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

        let next_step = &self.steps[self.current_index];
        self.current_index += 1;

        next_step.as_animation_frame(lights, &self.hold_time)
    }
}

pub struct DynamicAnimationStep {
    pub transforms: HashMap<LightNumber, LightStateTransform>,
}

impl DynamicAnimationStep {
    pub fn as_animation_frame(&self, lights: &LightCollection, hold_time: &Duration) -> AnimationFrame {
        let mut states: HashMap<LightNumber, LightState> = HashMap::new();

        for (light_num, transform) in &self.transforms {
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

#[derive(Default)]
pub struct LightStateTransform {
    pub on: Option<LightOnStateOperation>,
    pub transition_time: Option<LightStateValueOperation<u16>>,
    pub hue: Option<LightStateValueOperation<u16>>,
    pub saturation: Option<LightStateValueOperation<u8>>,
    pub brightness: Option<LightStateValueOperation<u8>>,
}

impl LightStateTransform {
    pub fn create_light_state(
        &self,
        light_num: LightNumber,
        previous_states: &LightCollection,
    ) -> LightState {
        let previous_state = previous_states.0.get(&light_num);

        let on = self.on.as_ref().and_then(|op| {
            op.process(
                previous_states,
                previous_state.and_then(|light| light.state.on),
            )
        });

        let transitiontime = self.transition_time.as_ref().and_then(|op| {
            op.process(
                previous_states,
                previous_state.and_then(|light| light.state.transitiontime),
            )
        });

        let hue = self.hue.as_ref().and_then(|op| {
            op.process(
                previous_states,
                previous_state.and_then(|light| light.state.hue),
            )
        });

        let sat = self.saturation.as_ref().and_then(|op| {
            op.process(
                previous_states,
                previous_state.and_then(|light| light.state.sat),
            )
        });

        let bri = self.brightness.as_ref().and_then(|op| {
            op.process(
                previous_states,
                previous_state.and_then(|light| light.state.bri),
            )
        });

        LightState {
            on,
            transitiontime,
            hue,
            sat,
            bri,
            ..Default::default()
        }
    }
}

pub enum LightOnStateOperation {
    Set(bool),
    Apply(Box<dyn Fn(&LightCollection, Option<bool>) -> Option<bool>>),
    Random,
    Toggle,
}

impl LightOnStateOperation {
    pub fn process(
        &self,
        previous_states: &LightCollection,
        previous_value: Option<bool>,
    ) -> Option<bool> {
        match self {
            LightOnStateOperation::Set(value) => Some(*value),
            LightOnStateOperation::Apply(func) => func(previous_states, previous_value),
            LightOnStateOperation::Random => thread_rng().gen(),
            LightOnStateOperation::Toggle => previous_value.map(|previous| !previous),
        }
    }
}

pub type LightStateValueFunction<T> =
    Box<dyn Fn(&LightCollection, Option<T>) -> Option<LightStateValue<T>>>;

pub enum LightStateValueOperation<T>
where
    T: Clone + Add + Mul + SampleUniform,
    Standard: Distribution<T>,
{
    Set(LightStateValue<T>),
    Add(LightStateValue<T>),
    Multiply(LightStateValue<T>),
    Apply(LightStateValueFunction<T>),
}

impl<T> LightStateValueOperation<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + SampleUniform,
    Standard: Distribution<T>,
{
    pub fn process(
        &self,
        previous_states: &LightCollection,
        previous_value: Option<T>,
    ) -> Option<T> {
        match self {
            LightStateValueOperation::Set(value) => Some(value.generate()),
            LightStateValueOperation::Add(value) => {
                previous_value.map(|previous| previous.clone() + value.generate())
            }
            LightStateValueOperation::Multiply(value) => {
                previous_value.map(|previous| previous.clone() * value.generate())
            }
            LightStateValueOperation::Apply(func) => {
                func(previous_states, previous_value).map(|value| value.generate())
            }
        }
    }
}

#[derive(Clone)]
pub enum LightStateValue<T>
where
    T: Clone + Add + Mul + SampleUniform,
    Standard: Distribution<T>,
{
    Constant(T),
    RandomRange(T, T),
    Random,
}

impl<T> LightStateValue<T>
where
    T: Clone + Add + Mul + SampleUniform,
    Standard: Distribution<T>,
{
    pub fn generate(&self) -> T {
        match self {
            LightStateValue::Constant(value) => value.clone(),
            LightStateValue::RandomRange(min, max) => {
                let mut rng = thread_rng();
                rng.gen_range(min.clone(), max.clone())
            }
            LightStateValue::Random => {
                let mut rng = thread_rng();
                rng.gen()
            }
        }
    }
}
