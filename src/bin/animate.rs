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

    let a = hoo::animation::from_current(&connection, &Duration::from_secs(5), &Duration::from_secs(0))?;
    a.play(&connection)?;

    Ok(())
}
