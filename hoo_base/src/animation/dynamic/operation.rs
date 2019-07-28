use rand::distributions::{uniform::SampleUniform, Distribution, Standard};
use rand::{thread_rng, Rng};

use std::ops::{Add, Mul};

use hoo_api::light::LightCollection;

use crate::animation::dynamic::BoxedValueProducer;

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
    Box<dyn Fn(&LightCollection, Option<T>) -> Option<BoxedValueProducer<T>>>;

pub enum LightStateValueOperation<T>
where
    T: Clone + Add + Mul + SampleUniform,
    Standard: Distribution<T>,
{
    Set(BoxedValueProducer<T>),
    Add(BoxedValueProducer<T>),
    Multiply(BoxedValueProducer<T>),
    Apply(LightStateValueFunction<T>),
}

impl<T> LightStateValueOperation<T>
where
    T: Clone + Add<Output = T> + Mul<Output = T> + SampleUniform,
    Standard: Distribution<T>,
{
    pub fn process(
        &mut self,
        previous_states: &LightCollection,
        previous_value: Option<T>,
    ) -> Option<T> {
        match self {
            LightStateValueOperation::Set(value) => Some(value.produce()),
            LightStateValueOperation::Add(value) => {
                previous_value.map(|previous| previous.clone() + value.produce())
            }
            LightStateValueOperation::Multiply(value) => {
                previous_value.map(|previous| previous.clone() * value.produce())
            }
            LightStateValueOperation::Apply(func) => {
                func(previous_states, previous_value).map(|mut value| value.produce())
            }
        }
    }
}
