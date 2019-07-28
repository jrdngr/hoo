use rand::distributions::{uniform::SampleUniform, Distribution, Standard};
use rand::{thread_rng, Rng};

pub type BoxedValueProducer<T> = Box<ValueProducer<T>>;

pub trait ValueProducer<T> {
    fn produce(&mut self) -> T;
}

#[derive(Debug, Clone)]
pub struct ConstantProducer<T> {
    value: T,
}

impl<T> ConstantProducer<T> {
    pub fn new(value: T) -> Box<Self> {
        Box::new(Self { value })
    }
}

impl<T: Clone> ValueProducer<T> for ConstantProducer<T> {
    fn produce(&mut self) -> T {
        self.value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct RandomProducer;

impl RandomProducer {
    pub fn new() -> Box<Self> {
        Box::new(RandomProducer)
    }
}

impl<T> ValueProducer<T> for RandomProducer
where
    Standard: Distribution<T>,
{
    fn produce(&mut self) -> T {
        thread_rng().gen()
    }
}

#[derive(Debug, Clone)]
pub struct RandomRangeProducer<T> {
    min: T,
    max: T,
}

impl<T> RandomRangeProducer<T> {
    pub fn new(min: T, max: T) -> Box<Self> {
        Box::new(Self { min, max })
    }
}

impl<T> ValueProducer<T> for RandomRangeProducer<T>
where
    T: SampleUniform,
    Standard: Distribution<T>,
{
    fn produce(&mut self) -> T {
        thread_rng().gen_range(&self.min, &self.max)
    }
}
