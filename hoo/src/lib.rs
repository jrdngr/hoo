use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use hoohue_api as api;
use hoohue_api::ApiConnection;

pub mod animation;
pub mod effects;

pub type AnyError = Box<dyn std::error::Error>;

type LightNumber = u8;
type RgbValue = f64;
type HueValue = u16;
type SaturationValue = u8;
type BrightnessValue = u8;
type TransitionTime = u16;
type HoldTime = u16;

pub struct Hoo {
    receiver: Receiver<HooCommand>,
    connection: ApiConnection,
}

impl Hoo {
    pub fn new() -> (Self, Sender<HooCommand>) {
        dotenv::dotenv().ok();

        let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
        let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

        let (sender, receiver) = mpsc::channel();

        (
            Hoo {
                receiver,
                connection: ApiConnection::new(&base_uri, &user_id),
            },
            sender,
        )
    }

    pub fn run(&self) {
        loop {
            if let Ok(msg) = self.receiver.try_recv() {
                println!("{:?}", msg);
                match msg {
                    HooCommand::On(light_num) => {
                        api::on(&self.connection, light_num);
                    }
                    HooCommand::Off(light_num) => {
                        api::off(&self.connection, light_num);
                    }
                    HooCommand::Quit => return,
                    _ => println!("nah"),
                }
            }
        }
    }
}

impl Default for Hoo {
    fn default() -> Self {
        Hoo::new().0
    }
}

#[derive(Debug, Clone)]
pub enum HooCommand {
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
    ColorLoop(LightNumber, bool),
    TransitionTime(LightNumber, TransitionTime),
    // Animate(TransitionTime, HoldTime),
    // Rainbow(Duration),
    // Random(TransitionTime, HoldTime),
    StopAnimation,
    Quit,
}
