use actix_web::web::{Data, Json, Path, Query};
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_cors::Cors;

use std::sync::mpsc::{self, Sender};
use std::time::Duration;

use hoo_api::light::{Light, LightCollection, LightState};
use hoo_base::{HooCommand, HooConfig};
pub use app_state::AppState;
pub use common::{RGB, AnimationSettings, HooResponse, HooError};

pub mod app_state;
pub mod common;

const TIMEOUT: Duration = Duration::from_secs(5);

pub struct HooServer;

impl HooServer {
    pub fn run(config: &HooConfig, sender: Sender<HooCommand>) -> Result<(), std::io::Error> {
        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::new()
                        .allowed_origin("http://localhost:8080")
                        .allowed_origin("http://127.0.0.1:8080")
                        .allowed_methods(vec!["GET", "POST"])
                )
                .data(AppState::new(&sender))
                .service(
                    web::scope("/api")
                        .service(web::resource("/stop").route(web::get().to(stop_animation)))
                        .service(
                            web::resource("/rotate/{trans_time}/{hold_time}")
                                .route(web::get().to(rotate)),
                        )
                        .service(
                            web::resource("/random/{trans_time}/{hold_time}")
                                .route(web::get().to(random)),
                        )
                        .service(web::resource("/animate").route(web::post().to(animate)))
                        .service(
                            web::resource("/light/{light_num}").route(web::get().to(get_light)),
                        )
                        .service(web::resource("/lights").route(web::get().to(get_all_lights)))
                        .service(
                            web::scope("/{light_num}")
                                .service(web::resource("/on").route(web::get().to(on)))
                                .service(web::resource("/off").route(web::get().to(off)))
                                .service(web::resource("/color").route(web::get().to(color)))
                                .service(web::resource("/state").route(web::get().to(light_state))),
                        ),
                )
                .service(
                    actix_files::Files::new("/", "./hoo_frontend/dist/").index_file("index.html"),
                )
        })
        .bind(&config.hoo_server_socket_uri)?
        .workers(1)
        .run()
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
        Err(_) => Err(HooError::default().into()),
    }
}

fn get_all_lights(state: Data<AppState>) -> Result<Json<LightCollection>> {
    let (sender, receiver) = mpsc::channel::<LightCollection>();
    let _ = state.sender.send(HooCommand::GetAllLights(sender));

    let response = receiver.recv_timeout(TIMEOUT);
    match response {
        Ok(lights) => Ok(Json(lights)),
        Err(_) => Err(HooError::default().into()),
    }
}
