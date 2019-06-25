use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{error, http, web, App, HttpResponse, HttpServer, Result};
use failure::Fail;
use serde::{Deserialize, Serialize};

use hoo_api::light::{Light, LightCollection, LightState};
use hoo_base::{Hoo, HooCommand};

const TIMEOUT: Duration = Duration::from_secs(5);

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let socket_ip = std::env::var("SOCKET_IP").expect("SOCKET_IP must be set");

    let (hoo, sender) = Hoo::new();

    thread::spawn(move || hoo.run());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .data(AppState::new(&sender))
            .service(web::resource("/stop").route(web::get().to(stop_animation)))
            .service(web::resource("/rotate/{trans_time}/{hold_time}").route(web::get().to(rotate)))
            .service(web::resource("/random/{trans_time}/{hold_time}").route(web::get().to(random)))
            .service(web::resource("/animate").route(web::post().to(animate)))
            // |r| {
            //     r.method(Method::POST).with_config(animate, |(_, cfg)| {
            //         cfg.error_handler(|err, req| {
            //             println!("{:?}", err);
            //             println!("{:?}", req);
            //             error::InternalError::from_response(err, HttpResponse::Conflict().finish())
            //                 .into()
            //         });
            //     })
            // }))
            .service(web::resource("/light/{light_num}").route(web::get().to(get_light)))
            .service(web::resource("/lights").route(web::get().to(get_all_lights)))
            .service(
                web::scope("/{light_num}")
                    .service(web::resource("/on").route(web::get().to(on)))
                    .service(web::resource("/off").route(web::get().to(off)))
                    .service(web::resource("/color").route(web::get().to(color)))
                    .service(web::resource("/state").route(web::get().to(light_state))),
            )
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(socket_ip)?
    .workers(1)
    .run()?;

    Ok(())
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

fn on(state: Data<AppState>, light_num: Path<u8>) -> HttpResponse {
    let _ = state.sender.send(HooCommand::On(*light_num));

    HttpResponse::Ok().json(HooResponse::default())
}

fn off(state: Data<AppState>, light_num: Path<u8>) -> HttpResponse {
    let _ = state.sender.send(HooCommand::Off(*light_num));

    HttpResponse::Ok().json(HooResponse::default())
}

#[derive(Debug, Deserialize)]
struct RGB {
    r: Option<u8>,
    g: Option<u8>,
    b: Option<u8>,
}

fn color(state: Data<AppState>, light_num: Path<u8>, color: Query<RGB>) -> HttpResponse {
    let r = color.r.unwrap_or(0);
    let g = color.g.unwrap_or(0);
    let b = color.b.unwrap_or(0);

    let _ = state.sender.send(HooCommand::RgbColor(*light_num, r, g, b));

    HttpResponse::Ok().json(HooResponse::default())
}

fn light_state(
    state: Data<AppState>,
    light_num: Path<u8>,
    light_state: Query<LightState>,
) -> HttpResponse {
    let _ = state
        .sender
        .send(HooCommand::State(*light_num, light_state.clone()));

    HttpResponse::Ok().json(HooResponse::default())
}

fn rotate(state: Data<AppState>, info: Path<(u16, u16)>) -> HttpResponse {
    let _ = state.sender.send(HooCommand::Rotate(info.0, info.1));

    HttpResponse::Ok().json(HooResponse::default())
}

fn random(state: Data<AppState>, info: Path<(u16, u16)>) -> HttpResponse {
    let _ = state.sender.send(HooCommand::Random(info.0, info.1));

    HttpResponse::Ok().json(HooResponse::default())
}

fn animate(state: Data<AppState>, data: Json<AnimationSettings>) -> HttpResponse {
    println!("data: {:?}", data);
    let _ = state
        .sender
        .send(HooCommand::Rotate(data.transition_time, data.hold_time));

    HttpResponse::Ok().json(HooResponse::default())
}

fn stop_animation(state: Data<AppState>) -> HttpResponse {
    let _ = state.sender.send(HooCommand::StopAnimation);

    HttpResponse::Ok().json(HooResponse::default())
}

fn get_light(state: Data<AppState>, light_num: Path<u8>) -> Result<Json<Light>> {
    let (sender, receiver) = mpsc::channel::<Light>();
    let _ = state.sender.send(HooCommand::GetLight(*light_num, sender));

    let response = receiver.recv_timeout(TIMEOUT);

    match response {
        Ok(light) => Ok(Json(light)),
        Err(_) => Err(HooError::new().into()),
    }
}

fn get_all_lights(state: Data<AppState>) -> Result<Json<LightCollection>> {
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
