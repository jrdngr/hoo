use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use hoo_api::color::Color;
use hoo_api::light::{Light, LightCollection, LightState};
use hoo_api::ApiConnection;


use crate::animation_old::effects::random::RandomAnimation;
use crate::animation_old::effects::rotate::RotateAnimation;
use crate::animation_old::AnimationFrame;
pub mod animation;
pub mod animation_old;
pub mod config;
// pub mod light_controller;

type LightNumber = u8;
type RgbValue = u8;
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
    #[allow(clippy::new_ret_no_self)]
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
                            RandomAnimation::new(&self.connection, &transition_time, &hold_time)
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
                        let response = self.connection.get_active_lights();
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
                                let delay = frame.transition_time + frame.hold_time;
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
    State(LightNumber, LightState),
    Rotate(TransitionTime, HoldTime),
    Rainbow(Duration),
    Random(TransitionTime, HoldTime),
    StopAnimation,
    GetLight(LightNumber, Sender<Light>),
    GetAllLights(Sender<LightCollection>),
    Quit,
}
