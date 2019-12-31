use anyhow::Result;
use std::path::Path;

use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use crate::animation::builtins::sleepy_random::create_sleepy_random_animation;
use crate::animation::builtins::random::create_random_animation;
// use crate::animation::builtins::rotate::RotateAnimation;
use crate::animation::DynamicAnimation;

use hoo_api::connection::ApiConnection;
use hoo_api_types::{Light, LightCollection, LightState, Color};

pub use crate::config::HooConfig;

pub mod animation;
pub mod config;

type LightNumber = u8;
type RgbValue = u8;
type HueValue = u16;
type SaturationValue = u8;
type BrightnessValue = u8;
type TransitionTime = u16;
type HoldTime = u16;

pub struct Hoo {
    config: HooConfig,
    receiver: Receiver<HooCommand>,
    connection: ApiConnection,
}

impl Hoo {
    pub fn with_config(config: HooConfig) -> (Self, Sender<HooCommand>) {
        let (sender, receiver) = mpsc::channel();
        let connection = ApiConnection::new(&config.hue_hub_uri, &config.hue_user_id);

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
    ) -> Result<(Self, Sender<HooCommand>)> {
        let config = HooConfig::from_file(file_path)?;
        Ok(Self::with_config(config))
    }

    pub fn config(&self) -> &HooConfig {
        &self.config
    }

    pub async fn run(&mut self) {
        let mut next_frame_time: Option<Instant> = None;
        let mut animation: Option<DynamicAnimation> = None;

        loop {
            if let Ok(msg) = self.receiver.try_recv() {
                println!("{:?}", msg);
                match msg {
                    HooCommand::On(light_num) => {
                        let _ = self.connection.on(light_num).await;
                    }
                    HooCommand::Off(light_num) => {
                        let _ = self.connection.off(light_num).await;
                    }
                    HooCommand::RgbColor(light_num, r, g, b) => {
                        let r = f64::from(r) / f64::from(std::u8::MAX);
                        let g = f64::from(g) / f64::from(std::u8::MAX);
                        let b = f64::from(b) / f64::from(std::u8::MAX);

                        let state = LightState::new().color(&Color::from_rgb(r, g, b)).sat(255);
                        let _ = self.connection.set_state(light_num, &state).await;
                    }
                    HooCommand::State(light_num, state) => {
                        let _ = self.connection.set_state(light_num, &state).await;
                    }
                    // HooCommand::Rotate(tt, ht) => {
                    //     let transition_time = Duration::from_secs(u64::from(tt));
                    //     let hold_time = Duration::from_secs(u64::from(ht));
                    //     animation = RotateAnimation::new(&self.connection, &transition_time, &hold_time).ok();
                    //     next_frame_time = Some(Instant::now());
                    // }
                    HooCommand::Random(tt, ht) => {
                        let transition_time = Duration::from_secs(u64::from(tt));
                        let hold_time = Duration::from_secs(u64::from(ht));
                        animation = create_random_animation(&self.connection, &transition_time, &hold_time).await.ok();
                        next_frame_time = Some(Instant::now());
                    }
                    HooCommand::SleepyRandom(tt, ht) => {
                        let transition_time = Duration::from_secs(u64::from(tt));
                        let hold_time = Duration::from_secs(u64::from(ht));
                        animation = create_sleepy_random_animation(&self.connection, &transition_time, &hold_time).await.ok();
                        next_frame_time = Some(Instant::now());
                    }
                    HooCommand::StopAnimation => next_frame_time = None,
                    HooCommand::GetLight(light_num, sender) => {
                        let response = self.connection.get_light(light_num).await;
                        if let Ok(light) = response {
                            let _ = sender.send(light);
                        }
                    }
                    HooCommand::GetAllLights(sender) => {
                        let response = self.connection.get_all_lights().await;
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
                            if let Some(frame) = anim.next().await {
                                let delay = frame.transition_time.unwrap_or(Duration::from_secs(0))
                                    + frame.hold_time;
                                next_frame_time = Some(now + delay);
                                for state in frame.states {
                                    let _ = self.connection.set_state(state.0, &state.1).await;
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
