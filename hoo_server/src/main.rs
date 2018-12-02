use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use actix_web::http::Method;
use actix_web::{server, App, Path, Query, Request, Result, State};
use serde_derive::Deserialize;

use hoo::animation::AnimationMessage;
use hoo::effects;
use hoohue_api::{self, ApiConnection};

fn main() {
    dotenv::dotenv().ok();

    server::new(|| {
        let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
        let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        App::with_state(AppState {
            sender,
            receiver,
            connection: ApiConnection::new(&base_uri, &user_id),
        })
        .resource("/on/{light_num}", |r| r.method(Method::GET).with(on))
        .resource("/off/{light_num}", |r| r.method(Method::GET).with(off))
        .resource("anim", |r| r.method(Method::GET).with(animate))
        .resource("stop", |r| r.method(Method::GET).with(stop_animation))
        .finish()
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}

struct AppState {
    sender: mpsc::Sender<AnimationMessage>,
    receiver: Arc<Mutex<mpsc::Receiver<AnimationMessage>>>,
    connection: ApiConnection,
}

fn on(state: State<AppState>, info: Path<u8>) -> Result<String> {
    hoohue_api::on(&state.connection, *info).expect("Error turning on light");
    Ok(format!("{} -> on", info))
}

fn off(state: State<AppState>, info: Path<u8>) -> Result<String> {
    hoohue_api::off(&state.connection, *info).expect("Error turning off light");
    Ok(format!("{} -> off", info))
}

#[derive(Deserialize)]
struct AnimateMessage {
    tt: Option<u64>,
    ht: Option<u64>,
}

fn animate(state: State<AppState>, info: Query<AnimateMessage>) -> Result<String> {
    let transition_time = Duration::from_secs(info.tt.unwrap_or(1));
    let hold_time = Duration::from_secs(info.ht.unwrap_or(0));

    let anim = effects::rotate_current(
        &state.connection,
        &transition_time,
        &hold_time,
        state.receiver.clone(),
    )
    .expect("Error creating animation");

    //thread::spawn(move || anim.play(&state.connection).expect("Error animating"));

    Ok(format!("{:?} {:?}", info.tt, info.ht))
}

fn stop_animation(state: State<AppState>) -> Result<String> {
    state.sender.send(AnimationMessage::Stop);

    Ok(format!("Stopping animation"))
}
