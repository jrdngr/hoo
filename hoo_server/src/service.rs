use anyhow::Result;
use hyper::{Body, Request, Response, StatusCode, service::Service};
use tokio::sync::mpsc::{self, Sender};

use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

pub use app_state::AppState;
pub use common::{AnimationSettings, HooResponse, RGB};
use hoo_api::light::{Light, LightCollection, LightState};
use hoo_base::HooCommand;

pub mod app_state;
pub mod common;

const TIMEOUT: Duration = Duration::from_secs(5);

pub struct HooService {
    sender: Sender<HooCommand>,
}

impl HooService {
    pub fn new(sender: Sender<HooCommand>) -> Self {
        Self { sender }
    }

    pub async fn handle(&self, request: Request<Body>) -> Result<Response<Body>> {
    
        Ok(Response::new("Heyo".into()))
    }
}

impl Service<Request<Body>> for HooService {
    type Response = Response<Body>;
    type Error = anyhow::Error;

    fn call(&mut self, req: Request<Vec<u8>>) -> Self::Future {
        // create the body
        let body: Vec<u8> = "hello, world!\n"
            .as_bytes()
            .to_owned();
        // Create the HTTP response
        let resp = Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .expect("Unable to create `http::Response`");
         
        // create a response in a future.
        let fut = async {
            Ok(resp)
        };

        // Return the response as an immediate future
        Box::pin(fut)
    }
}

// #[get("/{light_num}/on")]
// async fn on(state: Data<AppState>, light_num: Path<u8>) -> HttpResponse {
//     let _ = state.sender.send(HooCommand::On(*light_num));
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/{light_num}/off")]
// async fn off(state: Data<AppState>, light_num: Path<u8>) -> HttpResponse {
//     let _ = state.sender.send(HooCommand::Off(*light_num));
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/{light_num}/color")]
// async fn color(state: Data<AppState>, light_num: Path<u8>, color: Query<RGB>) -> HttpResponse {
//     let r = color.r.unwrap_or(0);
//     let g = color.g.unwrap_or(0);
//     let b = color.b.unwrap_or(0);

//     let _ = state.sender.send(HooCommand::RgbColor(*light_num, r, g, b));
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/{light_num}/state")]
// async fn light_state(
//     state: Data<AppState>,
//     light_num: Path<u8>,
//     light_state: Query<LightState>,
// ) -> HttpResponse {
//     let _ = state
//         .sender
//         .send(HooCommand::State(*light_num, light_state.clone()));

//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/rotate/{trans_time}/{hold_time}")]
// async fn rotate(state: Data<AppState>, info: Path<(u16, u16)>) -> HttpResponse {
//     let _ = state.sender.send(HooCommand::Rotate(info.0, info.1));
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/random/{trans_time}/{hold_time}")]
// async fn random(state: Data<AppState>, info: Path<(u16, u16)>) -> HttpResponse {
//     let _ = state.sender.send(HooCommand::Random(info.0, info.1));
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/sleepy/{trans_time}/{hold_time}")]
// async fn sleepy(state: Data<AppState>, info: Path<(u16, u16)>) -> HttpResponse {
//     let _ = state.sender.send(HooCommand::SleepyRandom(info.0, info.1));
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/animate")]
// async fn animate(state: Data<AppState>, data: Json<AnimationSettings>) -> HttpResponse {
//     println!("data: {:?}", data);
//     let _ = state
//         .sender
//         .send(HooCommand::Rotate(data.transition_time, data.hold_time));

//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/stop")]
// async fn stop_animation(state: Data<AppState>) -> HttpResponse {
//     let _ = state.sender.send(HooCommand::StopAnimation);
//     HttpResponse::Ok().json(HooResponse::default())
// }

// #[get("/light/{light_num}")]
// async fn get_light(state: Data<AppState>, light_num: Path<u8>) -> Result<HttpResponse, actix_web::Error> {
//     let (sender, receiver) = mpsc::channel::<Light>();
//     let _ = state.sender.send(HooCommand::GetLight(*light_num, sender));

//     let light = receiver
//         .recv_timeout(TIMEOUT)
//         .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

//     Ok(HttpResponse::Ok().json(light))
// }

// #[get("/lights")]
// async fn get_all_lights(state: Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
//     let (sender, receiver) = mpsc::channel::<LightCollection>();
//     let _ = state.sender.send(HooCommand::GetAllLights(sender));

//     let lights = receiver
//         .recv_timeout(TIMEOUT)
//         .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

//     Ok(HttpResponse::Ok().json(lights))
// }
