use anyhow::Result;
use std::time::Duration;

use hoo_api::light::LightNumber;
use hoo_api::ApiConnection;

use crate::animation::dynamic::producer::{constant, random_range};
use crate::animation::dynamic::{ConfigurableValue, DynamicAnimation, DynamicAnimationStep};

pub async fn create_sleepy_random_animation<'a>(
    connection: &'a ApiConnection,
    transition_time: &Duration,
    hold_time: &Duration,
) -> Result<DynamicAnimation<'a>> {
    use ConfigurableValue::*;

    let mut animation = DynamicAnimation::new(connection, hold_time)?;

    let transition_millis =
        transition_time.as_secs() * 1000 + u64::from(transition_time.subsec_millis());
    let transition_hue_units = (transition_millis / 100) as u16;

    let lights = connection.get_active_lights().await?.clone();

    let mut operations: Vec<(LightNumber, ConfigurableValue)> = Vec::new();
    for light_num in lights.keys() {
        operations.push((*light_num, Hue(random_range(35000, 48000))));
        operations.push((*light_num, Saturation(random_range(200, 255))));
        operations.push((*light_num, TransitionTime(constant(transition_hue_units))));
    }

    let step = DynamicAnimationStep { operations };

    animation.animation_step(step);

    Ok(animation)
}
