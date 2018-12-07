use std::sync::mpsc::Sender;
use std::thread;

use actix_web::http::Method;
use actix_web::{fs::NamedFile, server, App, Path, Query, Result, State, Json, error, HttpResponse};
use serde_derive::Deserialize;

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
            .resource("/rotate/{trans_time}/{hold_time}", |r| {
                r.method(Method::GET).with(rotate)
            })
            .resource("/random/{trans_time}/{hold_time}", |r| {
                r.method(Method::GET).with(random)
            })
            .resource("/animate", |r| r.method(Method::POST).with_config(animate, |(_, cfg)| {
                cfg.error_handler(|err, req| {
                    println!("{:?}", err);
                    println!("{:?}", req);
                    error::InternalError::from_response(
                         err, HttpResponse::Conflict().finish()).into()
                });
            }))
            .resource("/stop", |r| r.method(Method::GET).with(stop_animation))
            .finish()
    })
    .bind(socket_ip)
    .unwrap()
    .run();
}

#[derive(Debug)]
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

fn controls(_state: State<AppState>) -> Result<NamedFile> {
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

fn rotate(state: State<AppState>, info: Path<(u16, u16)>) -> Result<NamedFile> {
    let _ = state.sender.send(HooCommand::Rotate(info.0, info.1));
    controls(state)
}

fn random(state: State<AppState>, info: Path<(u16, u16)>) -> Result<NamedFile> {
    let _ = state.sender.send(HooCommand::Random(info.0, info.1));
    controls(state)
}

fn animate(state: State<AppState>, data: Json<AnimationSettings>) -> Result<NamedFile> {
    println!("{:?}", data);
    let _ = state.sender.send(HooCommand::Rotate(data.transition_time, data.hold_time));
    controls(state)
}

fn stop_animation(state: State<AppState>) -> Result<NamedFile> {
    let _ = state.sender.send(HooCommand::StopAnimation);
    controls(state)
}

#[derive(Debug, Deserialize)]
struct AnimationSettings {
    transition_time: u16,
    hold_time: u16,
}