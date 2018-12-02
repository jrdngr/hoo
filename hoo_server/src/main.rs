use actix_web::http::Method;
use actix_web::{server, App, Path, Request, Result, State};
use serde_derive::Deserialize;

use hoohue_api::{api, api::ApiConnection};

fn main() {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    server::new(move || {
        App::with_state(AppState {
            connection: ApiConnection::new(&base_uri, &user_id),
        })
        .resource("/on/{light_num}", |r| r.method(Method::GET).with(on))
        .resource("/off/{light_num}", |r| r.method(Method::GET).with(off))
        .finish()
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}

struct AppState {
    connection: ApiConnection,
}

fn on(state: State<AppState>, info: Path<u8>) -> Result<String> {
    api::on(&state.connection, *info).expect("Error turning on light");
    Ok(format!("{} -> on", info))
}

fn off(state: State<AppState>, info: Path<u8>) -> Result<String> {
    api::off(&state.connection, *info).expect("Error turning off light");
    Ok(format!("{} -> off", info))
}
