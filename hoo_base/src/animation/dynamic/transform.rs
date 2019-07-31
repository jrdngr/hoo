use std::collections::HashMap;

use hoo_api::{LightCollection, LightNumber, LightState};

use crate::animation::dynamic::ConfigurableValue;

pub fn transform(
    mut lights: LightCollection,
    operations: &mut [(LightNumber, ConfigurableValue)],
) -> HashMap<LightNumber, LightState> {
    let mut result = HashMap::new();

    for (light_num, operation) in operations {
        operation.apply(&mut lights, *light_num);
    }

    for (light_num, light) in lights {
        result.insert(light_num, light.state);
    }

    result
}
