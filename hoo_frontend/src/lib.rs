use anyhow::{anyhow, Result};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use wasm_bindgen::prelude::*;

use hoo_api_types::{Light, LightNumber, LightCollection};

use std::collections::HashMap;

struct App {
    lights: LightCollection,
    fetch_service: FetchService,
    link: ComponentLink<Self>,
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
            _ => false,
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <h1>{"Hey there"}</h1>
        }
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_self(AppMessage::GetAllLights);
        
        Self { 
            lights: HashMap::new(),
            fetch_service: FetchService::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMessage::GetAllLights => {

                false
            },
            AppMessage::GetAllLightsResult(lights)=> {
                self.lights = lights;
                true
            },
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <h2>{"Ho there!"}</h2>
        }
    }
}

fn get_all_lights(fetch_service: &FetchService, callback: Callback<Result<LightCollection>>) -> FetchTask {
    let url = "http://localhost:3000/api/lights";

    let handler = move |response: Response<Json<Result<LightCollection>>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data)
        } else {
            callback.emit(Err(anyhow!(
                "{}: error getting lights",
                meta.status
            )))
        }
    };
    let request = Request::get(url).body(Nothing).unwrap();
    fetch_service.fetch(request, handler.into())
}

#[wasm_bindgen]
pub fn run() {
    yew::start_app::<App>();
}
