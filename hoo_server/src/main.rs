use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use actix_web::http::Method;
use actix_web::{fs::NamedFile, server, App, HttpRequest, Path, Query, Request, Result, State};
use serde_derive::Deserialize;

use hoo::animation::AnimationMessage;
use hoo::effects;
use hoo::{Hoo, HooCommand};
use hoohue_api::light::LightState;

fn main() {
    dotenv::dotenv().ok();

    let socket_ip = std::env::var("SOCKET_IP").expect("SOCKET_IP must be set");

    let (hoo, sender) = Hoo::new();

    thread::spawn(move || hoo.run());

    server::new(move || {
        App::with_state(AppState::new(&sender))
            .resource("/", |r| r.method(Method::GET).with(controls))
            .resource("/on/{light_num}", |r| r.method(Method::GET).with(on))
            .resource("/off/{light_num}", |r| r.method(Method::GET).with(off))
            .resource("/color/{light_num}", |r| r.method(Method::GET).with(color))
            .resource("/state/{light_num}", |r| {
                r.method(Method::GET).with(light_state)
            })
            .finish()
    })
    .bind(format!("{}:8080", socket_ip))
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

fn controls(state: State<AppState>) -> Result<NamedFile> {
    let path = std::path::Path::new("./static/controls.html");

    let result = NamedFile::open(path);

    match &result {
        Ok(f) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }

    Ok(result?)
}

fn on(state: State<AppState>, light_num: Path<u8>) -> Result<NamedFile> {
    let _ = state.sender.send(HooCommand::On(*light_num));

    controls(state)
}

fn off(state: State<AppState>, light_num: Path<u8>) -> Result<NamedFile> {
    let _ = state.sender.send(HooCommand::Off(*light_num));

    controls(state)
}

#[derive(Debug, Deserialize)]
struct RGB {
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
}

fn color(state: State<AppState>, light_num: Path<u8>, color: Query<RGB>) -> Result<NamedFile> {
    let r = color.r.unwrap_or(0);
    let g = color.g.unwrap_or(0);
    let b = color.b.unwrap_or(0);

    let _ = state.sender.send(HooCommand::RgbColor(*light_num, r, g, b));

    controls(state)
}

fn light_state(
    state: State<AppState>,
    light_num: Path<u8>,
    light_state: Query<LightState>,
) -> Result<NamedFile> {
    let _ = state
        .sender
        .send(HooCommand::State(*light_num, light_state.clone()));

    controls(state)
}
