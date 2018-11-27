use std::time::Duration;

use hoo::AnyError;
use hoo::light::LightState;
use hoo::animation::{Animation, AnimationFrame};
use hoo::api;
use hoo::color::Color;

fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let connection = hoo::api::ApiConnection::new(&base_uri, &user_id);

    let light_number: u8 = 2;

    api::on(&connection, light_number)?;

    let hold = Duration::from_secs(0);
    let transition = Duration::from_secs(10);

    let frames = vec![
        AnimationFrame {
            light_number,
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            state: LightState::new().color(&Color::from_hsv(50210, 222, 127)),
        },
        AnimationFrame {
            light_number,
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            state: LightState::new().color(&Color::from_hsv(46973, 217, 127)),
        },
        AnimationFrame {
            light_number,
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            state: LightState::new().color(&Color::from_hsv(46014, 254, 127)),
        },
        AnimationFrame {
            light_number,
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            state: LightState::new().color(&Color::from_hsv(46973, 217, 127)),
        },
    ];

    let animation = Animation::new().with_frames(frames);

    animation.play(&connection)?;

    Ok(())
}
