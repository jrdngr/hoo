use std::io::stdin;
use std::str::FromStr;

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
 
    let mut buffer = String::new();

    let light = api::get_light(&connection, 1)?;
    let mut current_color = match light.color() {
        Some(color) => color,
        None => Color::from_rgb(0.0, 0.0, 0.0),
    };

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Failed to read line");
        buffer = buffer.trim().to_string();

        let command = Command::from_str(&buffer)?;

        match command {
            Command::On(l) => { api::on(&connection, l)?; },
            Command::Off(l) => { api::off(&connection, l)?; },
            Command::Red(l, r) => {
                let (_, g, b) = current_color.rgb();
                current_color = Color::from_rgb(r, g, b);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::Green(l, g) => {
                let (r, _, b) = current_color.rgb();
                current_color = Color::from_rgb(r, g, b);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::Blue(l, b) => {
                let (r, g, _) = current_color.rgb();
                current_color = Color::from_rgb(r, g, b);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::RgbColor(l, r, g, b) => { 
                let red = r as f64 / std::u8::MAX as f64;
                let green = g as f64 / std::u8::MAX as f64;
                let blue = b as f64 / std::u8::MAX as f64;
                current_color = Color::from_rgb(red, green, blue);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::Hue(l, h) => {
                let (_, s, v) = current_color.hsv();
                current_color = Color::from_hsv(h, s, v);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::Saturation(l, s) => {
                let (h, _, v) = current_color.hsv();
                current_color = Color::from_hsv(h, s, v);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::Brightness(l, v) => {
                let (h, s, _) = current_color.hsv();
                current_color = Color::from_hsv(h, s, v);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::HsvColor(l, h, s, v) => {
                current_color = Color::from_hsv(h, s, v);
                let state = LightState::new().color(&current_color);
                api::set_state(&connection, l, &state)?;
            },
            Command::Invalid => println!("Invalid command"),
            Command::Quit => break,
            _ => println!("oops"),
        }
    }

    Ok(())
}

enum Command {
    On(LightNumber),
    Off(LightNumber),
    Red(LightNumber, RgbValue),
    Green(LightNumber, RgbValue),
    Blue(LightNumber, RgbValue),
    RgbColor(LightNumber, RgbValue, RgbValue, RgbValue),
    Hue(LightNumber, HueValue),
    Saturation(LightNumber, SaturationValue),
    Brightness(LightNumber, BrightnessValue),
    HsvColor(LightNumber, HueValue, SaturationValue, BrightnessValue),
    Quit,
    Invalid,
}

impl FromStr for Command {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "quit" || s == "q" {
            return Ok(Command::Quit);
        }

        let split = s.split(' ').collect::<Vec<&str>>();

        let light_num = split[0].parse::<u8>()?;
        let command = split[1];

        match command {
            "on" => Ok(Command::On(light_num)),
            "off" => Ok(Command::Off(light_num)),
            "red" => {
                let r = split[2].parse::<f64>()?;
                let r = r.min(1.0).max(0.0);
                Ok(Command::Red(light_num, r))
            },
            "green" => {
                let g = split[2].parse::<f64>()?;
                let g = g.min(1.0).max(0.0);
                Ok(Command::Green(light_num, g))
            },
            "blue" => {
                let b = split[2].parse::<f64>()?;
                let b = b.min(1.0).max(0.0);
                Ok(Command::Blue(light_num, b))
            },
            "rgb" => {
                let r = split[2].parse::<f64>()?;
                let r = r.min(1.0).max(0.0);
                let g = split[3].parse::<f64>()?;
                let g = g.min(1.0).max(0.0);
                let b = split[4].parse::<f64>()?;
                let b = b.min(1.0).max(0.0);

                Ok(Command::RgbColor(light_num, r, g, b))
            },
            "hue" => {
                let mut h = split[2].parse::<f64>()?;
                h = h % 360.0;
                if h < 0.0 {
                    h += 360.0;
                }
                let hue = ((h / 360.0) * std::u16::MAX as f64) as u16;

                Ok(Command::Hue(light_num, hue))
            },
            "sat" => {
                let mut s = split[2].parse::<f64>()?;
                s = s.min(1.0).max(0.0);
                let saturation = (s * std::u8::MAX as f64) as u8;

                Ok(Command::Saturation(light_num, saturation))
            },
            "bri" => {
                let mut v = split[2].parse::<f64>()?;
                v = v.min(1.0).max(0.0);
                let value = (v * std::u8::MAX as f64) as u8;

                Ok(Command::Brightness(light_num, value))
            },
            "hsv" => {
                let mut h = split[2].parse::<f64>()?;
                h = h % 360.0;
                if h < 0.0 {
                    h += 360.0;
                }

                let mut s = split[3].parse::<f64>()?;
                s = s.min(1.0).max(0.0);

                let mut v = split[4].parse::<f64>()?;
                v = v.min(1.0).max(0.0);

                let hue = ((h / 360.0) * std::u16::MAX as f64) as u16;
                let saturation = (s * std::u8::MAX as f64) as u8;
                let value = (v * std::u8::MAX as f64) as u8;

                Ok(Command::HsvColor(light_num, hue, saturation, value))
            },
            _ => Ok(Command::Invalid),
        }
    }
}