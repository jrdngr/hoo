use std::time::Duration;

use crate::AnyError;
use crate::api::{ApiConnection, get_active_lights};
use crate::light::LightState;
use crate::animation::{Animation, AnimationFrame};

pub fn rotate_current(connection: &ApiConnection, transition_time: &Duration, hold_time: &Duration) -> Result<Animation, AnyError> {
    let all_lights = get_active_lights(connection)?.0;

    let mut active_lights = Vec::new();
    let mut light_states = Vec::new();

    for (light_num, light) in all_lights {
         if let Some(color) = light.state.get_color() {
            active_lights.push(light_num);
            light_states.push(LightState::new().color(&color));
        }
    }

    let mut frames = Vec::new();

    let num_lights = light_states.len();

    for _ in 0 .. num_lights {
        light_states.rotate_right(1);

        let active_lights_copy = active_lights.clone();
        let light_states_copy = light_states.clone();
        
        let frame = AnimationFrame {
            hold_time: hold_time.clone(),
            transition_time: transition_time.clone(),
            states: active_lights_copy.into_iter().zip(light_states_copy).collect(),
        };

        frames.push(frame);
    }

    Ok(Animation::new().with_frames(frames))
}

pub fn rainbow(connection: &ApiConnection, time_per_loop: &Duration) -> Result<Animation, AnyError> {
    let minimum_time_per_loop = Duration::from_secs(3);
    let transition_time = Duration::from_millis(500);

    let lights = get_active_lights(connection)?.0;

    let mut frames = Vec::new();

    let time_per_loop = if *time_per_loop < minimum_time_per_loop {
        minimum_time_per_loop
    } else {
        time_per_loop.clone()
    };

    let time_millis = (time_per_loop.as_secs() * 1000) + time_per_loop.subsec_millis() as u64;
    let transition_time_millis = (transition_time.as_secs() * 1000) + transition_time.subsec_millis() as u64;

    let number_of_steps = time_millis / transition_time_millis;
    let transition_time = Duration::from_millis(time_millis / number_of_steps);
    let number_of_lights = lights.len();
    let hue_step_size = std::u16::MAX / number_of_steps as u16;
    let next_light_step_size = std::u16::MAX / number_of_lights as u16;

    for i in 0..number_of_steps {
        let mut states = Vec::new();

        let mut current_hue: u16 = (i as u16 * hue_step_size) as u16;

        for light_num in lights.keys() {
            let state = LightState::new().hue(current_hue).sat(255);
            states.push((*light_num, state));
            
            current_hue = current_hue.wrapping_add(next_light_step_size);
        }

        let frame = AnimationFrame {
            hold_time: Duration::from_secs(0),
            transition_time: transition_time.clone(),
            states,
        };

        frames.push(frame);
    }

    Ok(Animation::new().with_frames(frames))
}