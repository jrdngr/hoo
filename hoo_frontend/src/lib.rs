use seed::{*, prelude::*};
use seed::browser::service::fetch;
use hoo_api_types::LightCollection;

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
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GetAllLights => { orders.skip().perform_cmd(get_all_lights()); },
        Msg::GetAllLightsFetched(Ok(lights)) => { 
            model.lights = lights;
        },
        Msg::GetAllLightsFetched(Err(_e)) => { orders.skip(); },
    };
}

fn view(model: &Model) -> impl View<Msg> {
    button![
        simple_ev(Ev::Click, Msg::GetAllLights),
        format!("Hello, World × {}", model.lights.len())
    ]
}

async fn get_all_lights() -> Result<Msg, Msg> {
    Request::new("http://localhost:3000/api/lights")
        .method(Method::Get)
        .fetch_json_data(Msg::GetAllLightsFetched)
        .await
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}
