use std::ops::{Add, Mul};

use rand::{Rng, thread_rng};
use rand::distributions::{uniform::SampleUniform, Distribution, Standard};

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
