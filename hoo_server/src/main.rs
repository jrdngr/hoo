use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use actix_web::http::Method;
use actix_web::{server, App, HttpRequest, Path, Query, Request, Result, State};
use serde_derive::Deserialize;

use hoo::animation::AnimationMessage;
use hoo::effects;
use hoo::{Hoo, HooCommand};

fn main() {
    dotenv::dotenv().ok();

    let (hoo, sender) = Hoo::new();

    thread::spawn(move || hoo.run());

    server::new(move || {
        App::with_state(AppState::new(&sender))
            .resource("/on/{light_num}", |r| r.method(Method::GET).with(on))
            .resource("/off/{light_num}", |r| r.method(Method::GET).with(off))
            .resource("/stop", |r| r.method(Method::GET).with(stop))
            .finish()
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}

struct AppState {
    sender: Sender<HooCommand>,
}

impl AppState {
    pub fn new(sender: &Sender<HooCommand>) -> Self {
        Self {
            sender: sender.clone(),
        }
    }
}

fn on(state: State<AppState>, info: Path<u8>) -> Result<String> {
    let _ = state.sender.send(HooCommand::On(*info));

    Ok(format!("{} -> on", info))
}

fn off(state: State<AppState>, info: Path<u8>) -> Result<String> {
    let _ = state.sender.send(HooCommand::Off(*info));

    Ok(format!("{} -> off", info))
}

fn stop(state: State<AppState>) -> Result<String> {
    state.sender.send(HooCommand::Quit);
    Ok("Stopping server".to_string())
}
