use anyhow::Result;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};

use hoo_api_types::{Light, LightNumber, LightCollection};

use std::collections::HashMap;

struct App {
    lights: LightCollection,
}

enum AppMessage {
    GetAllLights,
    GetAllLightsResult(LightCollection),
}

enum LightMessage {
    ToggleLight(LightNumber),
    ToggleLightResult(String),
}

struct LightComponent {
    // light: Light,
}

impl Component for LightComponent {
    type Message = LightMessage;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LightMessage::ToggleLight(light_num) => {
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <h1>Hey there</h1>
        }
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { lights: HashMap::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMessage::GetAllLights => {

                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <h2>"Ho there!"</h2>
        }
    }
}

async fn get_all_lights() -> Result<LightCollection> {
    let mut options = RequestInit::new();
    options.method("GET");
    let request = Request::new_with_str_and_init(
        "http://localhost:3000/api/lights",
        &options,
    ).unwrap();

    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text().unwrap()).await.unwrap();
    let text = text.as_string().unwrap();

    Ok(serde_json::from_str(&text)?)
}

#[wasm_bindgen]
pub fn run() {
    yew::start_app::<App>();
}
