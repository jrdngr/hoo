use http::Request;

use crate::{LightState, LightEffect};

fn get(uri: &str) -> Request<()> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .body(())
        .unwrap()
}

fn put<T>(uri: &str, body: T) -> Request<T> {
    Request::builder()
        .method("PUT")
        .uri(uri)
        .body(body)
        .unwrap()
}

pub fn get_all_lights(base_uri: &str) -> Request<()> {
    get(&format!("{}/lights", base_uri))
}

pub fn get_light(base_uri: &str, light_number: u8) -> Request<()> {
    get(&format!("{}/lights/{}", base_uri, light_number))
}

pub fn put_state(base_uri: &str, light_number: u8, state: LightState) -> Request<LightState> {
    let uri = format!("{}/lights/{}/state",base_uri, light_number);
    put(&uri, state)
}

pub fn on(base_uri: &str, light_number: u8) -> Request<LightState> {
    let state = LightState::new().on(true);
    put_state(base_uri, light_number, state)
}

pub fn off(base_uri: &str, light_number: u8) -> Request<LightState> {
    let state = LightState::new().on(false);
    put_state(base_uri, light_number, state)
}

pub fn colorloop(base_uri: &str, light_number: u8, enabled: bool) -> Request<LightState> {
    let effect = if enabled {
        LightEffect::ColorLoop
    } else {
        LightEffect::None
    };
    let state = LightState::new().effect(effect);
    put_state(base_uri, light_number, state)
}

pub fn transition_time(
    base_uri: &str,
    light_number: u8,
    transition_time: u16,
) -> Request<LightState> {
    let state = LightState::new().transitiontime(transition_time);
    put_state(base_uri, light_number, state)
}    