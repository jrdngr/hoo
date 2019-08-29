use std::error::Error;
use std::path::Path;

use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use crate::animation::builtins::sleepy_random::create_sleepy_random_animation;
use crate::animation::builtins::random::create_random_animation;
use crate::animation::builtins::rotate::RotateAnimation;
use crate::animation::AnimationFrame;

use hoo_api::color::Color;
use hoo_api::connection::standard::StandardApiConnection;
use hoo_api::connection::testing::TestingApiConnection;

use hoo_api::light::{Light, LightCollection, LightState};
use hoo_api::ApiConnection;

pub use crate::config::HooConfig;

pub mod animation;
pub mod config;
// Some day I'll play with actix
// pub mod light_controller;

type LightNumber = u8;
type RgbValue = u8;
type HueValue = u16;
type SaturationValue = u8;
type BrightnessValue = u8;
type TransitionTime = u16;
type HoldTime = u16;

pub struct Hoo<T: ApiConnection> {
    config: HooConfig,
    receiver: Receiver<HooCommand>,
    connection: T,
}

impl Hoo<StandardApiConnection> {
    pub fn with_config(config: HooConfig) -> (Self, Sender<HooCommand>) {
        let (sender, receiver) = mpsc::channel();
        let connection = StandardApiConnection::new(&config.hue_hub_uri, &config.hue_user_id);

        (
            Hoo {
                config,
                receiver,
                connection,
            },
            sender,
        )
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> (Self, Sender<HooCommand>) {
        let config = HooConfig::from_dotenv();
        Self::with_config(config)
    }

    pub fn with_config_file<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<(Self, Sender<HooCommand>), Box<dyn Error>> {
        let config = HooConfig::from_file(file_path)?;
        Ok(Self::with_config(config))
    }
}

impl Hoo<TestingApiConnection> {
    pub fn from_file<P: AsRef<std::path::Path>>(file_path: P) -> (Self, Sender<HooCommand>) {
        let config = HooConfig::default();

        let (sender, receiver) = mpsc::channel();
        let connection = TestingApiConnection::new(file_path).unwrap();

        (
            Hoo {
                config,
                receiver,
                connection,
            },
            sender,
        )
    }
}

impl<T: ApiConnection> Hoo<T> {
    pub fn config(&self) -> &HooConfig {
        &self.config
    }

    pub fn run(&self) {
        let mut next_frame_time: Option<Instant> = None;
        let mut animation: Option<Box<dyn Iterator<Item = AnimationFrame>>> = None;

        loop {
            if let Ok(msg) = self.receiver.try_recv() {
                println!("{:?}", msg);
                match msg {
                    HooCommand::On(light_num) => {
                        let _ = self.connection.on(light_num);
                    }
                    HooCommand::Off(light_num) => {
                        let _ = self.connection.off(light_num);
                    }
                    HooCommand::RgbColor(light_num, r, g, b) => {
                        let r = f64::from(r) / f64::from(std::u8::MAX);
                        let g = f64::from(g) / f64::from(std::u8::MAX);
                        let b = f64::from(b) / f64::from(std::u8::MAX);

                        let state = LightState::new().color(&Color::from_rgb(r, g, b)).sat(255);
                        let _ = self.connection.set_state(light_num, &state);
                    }
                    HooCommand::State(light_num, state) => {
                        let _ = self.connection.set_state(light_num, &state);
                    }
                    HooCommand::Rotate(tt, ht) => {
                        let transition_time = Duration::from_secs(u64::from(tt));
                        let hold_time = Duration::from_secs(u64::from(ht));
                        let anim =
                            RotateAnimation::new(&self.connection, &transition_time, &hold_time)
                                .unwrap();
                        animation = Some(Box::new(anim));
                        next_frame_time = Some(Instant::now());
                    }
                    HooCommand::Random(tt, ht) => {
                        let transition_time = Duration::from_secs(u64::from(tt));
                        let hold_time = Duration::from_secs(u64::from(ht));
                        let anim =
                            create_random_animation(&self.connection, &transition_time, &hold_time)
                                .unwrap();
                        animation = Some(Box::new(anim));
                        next_frame_time = Some(Instant::now());
                    }
                    HooCommand::SleepyRandom(tt, ht) => {
                        let transition_time = Duration::from_secs(u64::from(tt));
                        let hold_time = Duration::from_secs(u64::from(ht));
                        let anim =
                            create_sleepy_random_animation(&self.connection, &transition_time, &hold_time)
                                .unwrap();
                        animation = Some(Box::new(anim));
                        next_frame_time = Some(Instant::now());
                    }
                    HooCommand::StopAnimation => next_frame_time = None,
                    HooCommand::GetLight(light_num, sender) => {
                        let response = self.connection.get_light(light_num);
                        if let Ok(light) = response {
                            let _ = sender.send(light);
                        }
                    }
                    HooCommand::GetAllLights(sender) => {
                        let response = self.connection.get_all_lights();
                        if let Ok(lights) = response {
                            let _ = sender.send(lights);
                        }
                    }
                    HooCommand::Quit => return,
                    _ => println!("Not implemented"),
                }
            }

            if let Some(time) = next_frame_time {
                let now = Instant::now();
                if now >= time {
                    match &mut animation {
                        Some(anim) => {
                            let frame = anim.next();
                            if let Some(frame) = frame {
                                let delay = frame.transition_time.unwrap_or(Duration::from_secs(0))
                                    + frame.hold_time;
                                next_frame_time = Some(now + delay);
                                for state in frame.states {
                                    let _ = self.connection.set_state(state.0, &state.1);
                                }
                            }
                        }
                        None => next_frame_time = None,
                    }
                }
            }
        }
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
    State(LightNumber, LightState),
    Rotate(TransitionTime, HoldTime),
    Rainbow(Duration),
    Random(TransitionTime, HoldTime),
    SleepyRandom(TransitionTime, HoldTime),
    StopAnimation,
    GetLight(LightNumber, Sender<Light>),
    GetAllLights(Sender<LightCollection>),
    Quit,
}
