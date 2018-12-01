use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

use crate::animation::{Animation, AnimationFrame};
use hoohue_api::api::{get_active_lights, set_state, ApiConnection};
use hoohue_api::light::LightState;
use crate::AnyError;

pub fn rotate_current(
    connection: &ApiConnection,
    transition_time: &Duration,
    hold_time: &Duration,
) -> Result<Animation, AnyError> {
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

    for _ in 0..num_lights {
        light_states.rotate_right(1);

        let active_lights_copy = active_lights.clone();
        let light_states_copy = light_states.clone();

        let frame = AnimationFrame {
            hold_time: *hold_time,
            transition_time: *transition_time,
            states: active_lights_copy
                .into_iter()
                .zip(light_states_copy)
                .collect(),
        };

        frames.push(frame);
    }

    Ok(Animation::new().with_frames(frames))
}

pub fn rainbow(
    connection: &ApiConnection,
    time_per_loop: &Duration,
) -> Result<Animation, AnyError> {
    let minimum_time_per_loop = Duration::from_secs(3);
    let transition_time = Duration::from_millis(500);

    let lights = get_active_lights(connection)?.0;

    let mut frames = Vec::new();

    let time_per_loop = if *time_per_loop < minimum_time_per_loop {
        minimum_time_per_loop
    } else {
        *time_per_loop
    };

    let time_millis = (time_per_loop.as_secs() * 1000) + u64::from(time_per_loop.subsec_millis());
    let transition_time_millis =
        (transition_time.as_secs() * 1000) + u64::from(transition_time.subsec_millis());

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
            transition_time,
            states,
        };

        frames.push(frame);
    }

    Ok(Animation::new().with_frames(frames))
}

pub fn random(
    connection: &ApiConnection,
    transition_time: &Duration,
    hold_time: &Duration,
) -> Result<(), AnyError> {
    let mut rng = rand::thread_rng();
    let transition_millis =
        transition_time.as_secs() * 1000 + u64::from(transition_time.subsec_millis());

    let transition_value = transition_millis as u16 / 100;

    let lights = get_active_lights(connection)?.0;

    loop {
        for light_num in lights.keys() {
            let next_hue: u16 = rng.gen();
            let state = LightState::new()
                .hue(next_hue)
                .sat(255)
                .transitiontime(transition_value);
            set_state(connection, *light_num, &state)?;
        }

        sleep(*hold_time + *transition_time);
    }
}
