use std::time::{Instant, Duration};
use std::thread::sleep;

use hoo::AnyError;
use hoo::light::LightState;
use hoo::api;
use hoo::color::Color;

type LightNumber = u8;
type RgbValue = f64;
type HueValue = u16;
type SaturationValue = u8;
type BrightnessValue = u8;

fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let connection = hoo::api::ApiConnection::new(&base_uri, &user_id);

    let transitions = vec![
        TransitionState { 
            color: Color::from_rgb(0.0, 0.0, 1.0),
            hold_time: 1,
            transition_time: 10,
        },
        TransitionState { 
            color: Color::from_rgb(0.0, 1.0, 1.0),
            hold_time: 1,
            transition_time: 10,
        },
        TransitionState { 
            color: Color::from_rgb(0.0, 1.0, 0.0),
            hold_time: 1,
            transition_time: 10,
        },
    ];

    api::on(&connection, 1);

    let state = LightState::new().transitiontime(0);
    api::set_state(&connection, 1, &state)?;

    loop {
        for transition in &transitions {
            let state = LightState::new()
                .color(&transition.color)
                .transitiontime(transition.transition_time * 10);
            api::set_state(&connection, 1, &state);
            sleep(Duration::from_secs(transition.transition_time as u64 + transition.hold_time));
        }
    }
}

struct TransitionState {
    pub color: Color,
    pub hold_time: u64,
    pub transition_time: u16,
}