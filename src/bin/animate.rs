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
    let brightness: u8 = 64;

    api::on(&connection, light_number)?;

    let hold = Duration::from_secs(0);
    let transition = Duration::from_secs(30);

    let red = LightState::new().color(&Color::from_hsv(50210, 222, brightness));
    let purple = LightState::new().color(&Color::from_hsv(46973, 217, brightness));
    let blue = LightState::new().color(&Color::from_hsv(46014, 254, brightness));

    let frames = vec![
        AnimationFrame {
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            states: vec![
                (1, red.clone()),
                (3, purple.clone()),
                (2, blue.clone()),
            ],
        },
        AnimationFrame {
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            states: vec![
                (1, blue.clone()),
                (3, red.clone()),
                (2, purple.clone()),
            ],
        },
        AnimationFrame {
            hold_time: hold.clone(),
            transition_time: transition.clone(),
            states: vec![
                (1, purple.clone()),
                (3, blue.clone()),
                (2, red.clone()),
            ],
        },
    ];

    let animation = Animation::new().with_frames(frames);

    animation.play(&connection)?;

    Ok(())
}
