use std::collections::HashMap;
use std::time::Duration;

use hoo_api::light::LightNumber;
use hoo_api::ApiConnection;

use crate::animation::dynamic::{
    DynamicAnimation, DynamicAnimationStep, LightStateTransform, LightStateValue,
    LightStateValueOperation,
};

pub fn create_random_animation<'a>(
    connection: &'a ApiConnection,
    transition_time: &Duration,
    hold_time: &Duration,
) -> Result<DynamicAnimation<'a>, failure::Error> {
    let mut animation = DynamicAnimation::new(connection, hold_time)?;

    let transition_millis =
        transition_time.as_secs() * 1000 + u64::from(transition_time.subsec_millis());
    let transition_hue_units = (transition_millis / 100) as u16;

    let lights = connection.get_active_lights()?.clone();

    let mut transforms: HashMap<LightNumber, LightStateTransform> = HashMap::new();
    for light_num in lights.0.keys() {
        let transform = LightStateTransform {
            hue: Some(LightStateValueOperation::Set(LightStateValue::Random)),
            saturation: Some(LightStateValueOperation::Set(LightStateValue::RandomRange(200, 255))),
            transition_time: Some(LightStateValueOperation::Set(LightStateValue::Constant(
                transition_hue_units,
            ))),
            ..Default::default()
        };

        transforms.insert(*light_num, transform);
    }

    let step = DynamicAnimationStep { transforms };

    animation.animation_step(step);

    Ok(animation)
}

// pub fn create_random_animation_ideal<'a>(
//     connection: &'a ApiConnection,
//     transition_time: &Duration,
//     hold_time: &Duration,
// ) -> Result<DynamicAnimation<'a>, failure::Error> {
//     DynamicAnimation::new(connection)
//         .step(DynamicAnimationStep::new()
//             .hold_time(hold_time)
//             .transition_time(transition_time)
//             .all_lights([
//                 Hue::set(Value::Random), 
//                 Sat::set(Value::RandomRange(200, 255))
//             ])
//             .build()
//         )
//         .build()
// }
