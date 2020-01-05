use seed::{*, prelude::*};
use seed::browser::service::fetch;
use hoo_api_types::{Light, LightNumber, LightCollection};

use std::collections::HashMap;

struct Model {
    pub lights: LightCollection,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            lights: HashMap::new(),
        }
    }
}

#[derive(Clone)]
enum Msg {
    GetAllLights,
    GetAllLightsFetched(fetch::ResponseDataResult<LightCollection>),
    ToggleLight(LightNumber),
    ToggleLightSent(fetch::ResponseDataResult<String>),
}

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(get_all_lights());
    AfterMount::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GetAllLights => { orders.skip().perform_cmd(get_all_lights()); },
        Msg::GetAllLightsFetched(Ok(lights)) => { 
            model.lights = lights;
        },
        Msg::GetAllLightsFetched(Err(_e)) => { orders.skip(); },
        Msg::ToggleLight(light_num) => { orders.skip().perform_cmd(toggle_light(light_num)); }
        _ => (),
    };
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        button![
            simple_ev(Ev::Click, Msg::GetAllLights),
            "Refresh",
        ],
        model.lights
            .iter()
            .map(light_component),
    ]
}

fn light_component((light_num, light): (&LightNumber, &Light)) -> Node<Msg> {
    div![
        style![
            St::Margin => "20px 10px",
        ],
        attrs!{At::Class => "light-component"},
        div![
            attrs!{At::Class => "light-info"},
            light.name,
        ],
        div![
            attrs!{At::Class => "light-controls"},
            button![
                simple_ev(Ev::Click, Msg::ToggleLight(*light_num)),
                "Toggle",
            ],    
        ],
    ]
}

async fn get_all_lights() -> Result<Msg, Msg> {
    Request::new("http://localhost:3000/api/lights")
        .method(Method::Get)
        .fetch_json_data(Msg::GetAllLightsFetched)
        .await
}

async fn toggle_light(light_num: LightNumber) -> Result<Msg, Msg> {
    let uri = format!("http://localhost:3000/api/light/{}/toggle", light_num);
    Request::new(uri)
        .method(Method::Put)
        .fetch_string_data(Msg::ToggleLightSent)
        .await
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
