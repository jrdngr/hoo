use std::default::Default;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use actix_web::http::Method;
use actix_web::{error, fs, http, server, App, HttpResponse, Json, Path, Query, Result, State};
use failure::Fail;
use serde::{Deserialize, Serialize};

use hoo_base::{Hoo, HooCommand};
use hoo_api::light::{Light, LightCollection, LightState};

type HooResult = Result<Json<HooResponse>>;

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() {
    dotenv::dotenv().ok();

    let socket_ip = std::env::var("SOCKET_IP").expect("SOCKET_IP must be set");

    let (hoo, sender) = Hoo::new();

    thread::spawn(move || hoo.run());

    server::new(move || {
        App::with_state(AppState::new(&sender))
            .resource("{light_num}/on", |r| r.method(Method::GET).with(on))
            .resource("{light_num}/off", |r| r.method(Method::GET).with(off))
            .resource("{light_num}/color", |r| r.method(Method::GET).with(color))
            .resource("{light_num}/state", |r| {
                r.method(Method::GET).with(light_state)
            })
            .resource("/rotate/{trans_time}/{hold_time}", |r| {
                r.method(Method::GET).with(rotate)
            })
            .resource("/random/{trans_time}/{hold_time}", |r| {
                r.method(Method::GET).with(random)
            })
            .resource("/animate", |r| {
                r.method(Method::POST).with_config(animate, |(_, cfg)| {
                    cfg.error_handler(|err, req| {
                        println!("{:?}", err);
                        println!("{:?}", req);
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    });
                })
            })
            .resource("/stop", |r| r.method(Method::GET).with(stop_animation))
            .resource("/light/{light_num}", |r| {
                r.method(Method::GET).with(get_light)
            })
            .resource("/lights", |r| r.method(Method::GET).with(get_all_lights))
            .handler(
                "/",
                fs::StaticFiles::new("./hoo-frontend/dist/")
                    .unwrap()
                    .index_file("index.html"),
            )
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

fn on(state: State<AppState>, light_num: Path<u8>) -> HooResult {
    let _ = state.sender.send(HooCommand::On(*light_num));

    Ok(Json(Default::default()))
}

fn off(state: State<AppState>, light_num: Path<u8>) -> HooResult {
    let _ = state.sender.send(HooCommand::Off(*light_num));

    Ok(Json(Default::default()))
}

#[derive(Debug, Deserialize)]
struct RGB {
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
}

fn color(state: State<AppState>, light_num: Path<u8>, color: Query<RGB>) -> HooResult {
    let r = color.r.unwrap_or(0);
    let g = color.g.unwrap_or(0);
    let b = color.b.unwrap_or(0);

    let _ = state.sender.send(HooCommand::RgbColor(*light_num, r, g, b));

    Ok(Json(Default::default()))
}

fn light_state(
    state: State<AppState>,
    light_num: Path<u8>,
    light_state: Query<LightState>,
) -> HooResult {
    let _ = state
        .sender
        .send(HooCommand::State(*light_num, light_state.clone()));

    Ok(Json(Default::default()))
}

fn rotate(state: State<AppState>, info: Path<(u16, u16)>) -> HooResult {
    let _ = state.sender.send(HooCommand::Rotate(info.0, info.1));

    Ok(Json(Default::default()))
}

fn random(state: State<AppState>, info: Path<(u16, u16)>) -> HooResult {
    let _ = state.sender.send(HooCommand::Random(info.0, info.1));

    Ok(Json(Default::default()))
}

fn animate(state: State<AppState>, data: Json<AnimationSettings>) -> HooResult {
    println!("data: {:?}", data);
    let _ = state
        .sender
        .send(HooCommand::Rotate(data.transition_time, data.hold_time));

    Ok(Json(Default::default()))
}

fn stop_animation(state: State<AppState>) -> HooResult {
    let _ = state.sender.send(HooCommand::StopAnimation);

    Ok(Json(Default::default()))
}

fn get_light(state: State<AppState>, light_num: Path<u8>) -> Result<Json<Light>> {
    let (sender, receiver) = mpsc::channel::<Light>();
    let _ = state.sender.send(HooCommand::GetLight(*light_num, sender));

    let response = receiver.recv_timeout(TIMEOUT);

    match response {
        Ok(light) => Ok(Json(light)),
        Err(_) => Err(HooError::new().into()),
    }
}

fn get_all_lights(state: State<AppState>) -> Result<Json<LightCollection>> {
    let (sender, receiver) = mpsc::channel::<LightCollection>();
    let _ = state.sender.send(HooCommand::GetAllLights(sender));

    let response = receiver.recv_timeout(TIMEOUT);

    match response {
        Ok(lights) => Ok(Json(lights)),
        Err(_) => Err(HooError::new().into()),
    }
}

#[derive(Debug, Deserialize)]
struct AnimationSettings {
    transition_time: u16,
    hold_time: u16,
}

#[derive(Debug, Serialize)]
struct HooResponse {
    message: String,
}

impl Default for HooResponse {
    fn default() -> Self {
        HooResponse {
            message: "success".to_string(),
        }
    }
}

#[derive(Fail, Debug)]
#[fail(display = "Internal server error")]
struct HooError {}

impl HooError {
    pub fn new() -> Self {
        HooError {}
    }
}

impl error::ResponseError for HooError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
