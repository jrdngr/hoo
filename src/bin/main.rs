use std::io::{stdin, Read};

use hoo::AnyError;
use hoo::light::{Light, LightState};
use hoo::api;
use hoo::api::{ApiConnection};
use hoo::color::Color;


fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let connection = hoo::api::ApiConnection::new(&base_uri, &user_id);
 
    let mut buffer = String::new();

    let light = api::get_light(&connection, 1)?;

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Failed to read line");
        buffer = buffer.trim().to_string();

        if buffer == "quit".to_string() || buffer == "q".to_string() {
            break;
        }

        let split = buffer.split(' ').collect::<Vec<&str>>();

        let light_num = split[0].parse::<u8>()?;
        let command = split[1];

        if command == "on".to_string() {
            api::on(&connection, light_num);
        } else if command == "off".to_string() {
            api::off(&connection, light_num);
        } else if command == "red".to_string() {
            let value = split[2].parse::<u8>()?;
            let value = value as f64 / std::u8::MAX as f64;

            if let Some(color) = light.color() {
                let (_, g, b) = color.rgb();
                let state = LightState::new().color(Color::from_rgb(value, g, b));
                api::set_state(&connection, light_num, &state);
            } else {
                let state = LightState::new().color(Color::from_rgb(value, 0.0, 0.0));
            }
        } else if command == "green".to_string() {
            let value = split[2].parse::<u8>()?;
            let value = value as f64 / std::u8::MAX as f64;

            if let Some(color) = light.color() {
                let (r, _, b) = color.rgb();
                let state = LightState::new().color(Color::from_rgb(r, value, b));
                api::set_state(&connection, light_num, &state);
            } else {
                let state = LightState::new().color(Color::from_rgb(0.0, value, 0.0));
            }
        } else if command == "blue".to_string() {
            let value = split[2].parse::<u8>()?;
            let value = value as f64 / std::u8::MAX as f64;

            if let Some(color) = light.color() {
                let (r, g, _) = color.rgb();
                let state = LightState::new().color(Color::from_rgb(r, g, value));
                api::set_state(&connection, light_num, &state);
            } else {
                let state = LightState::new().color(Color::from_rgb(0.0, 0.0, value));
            }
        } else if command == "color".to_string() {
            let r = split[2].parse::<u8>()?;
            let r = r as f64 / std::u8::MAX as f64;

            let g = split[3].parse::<u8>()?;
            let g = g as f64 / std::u8::MAX as f64;

            let b = split[4].parse::<u8>()?;
            let b = b as f64 / std::u8::MAX as f64;

            let state = LightState::new().color(Color::from_rgb(r, g, b));
            api::set_state(&connection, light_num, &state);
        } else {
            println!("Unknown command");
        }
    }

    Ok(())
}