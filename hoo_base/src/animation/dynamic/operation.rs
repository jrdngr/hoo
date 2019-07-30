use hoo_api::LightCollection;

use crate::animation::dynamic::ValueProducer;

pub type OperationFunction<T> =
    Box<dyn Fn(&LightCollection, Option<T>) -> Option<Box<ValueProducer<T>>>>;

pub enum Operation<T> {
    Set(Box<ValueProducer<T>>),
    Map(OperationFunction<T>),
}

impl <T> Operation<T> {
    pub fn apply(
        &mut self,
        previous_states: &LightCollection,
        previous_value: Option<T>,
    ) -> Option<T> {
        match self {
            Operation::Set(value) => Some(value.produce()),
            Operation::Map(func) => {
                func(previous_states, previous_value).map(|mut value| value.produce())
            }
        }
    }
}
