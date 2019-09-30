use failure::Error;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;
use hoo_api_types::LightCollection;

use crate::api::HooApi;

pub struct App {
    lights: Option<LightCollection>,
    api: HooApi,
    lights_callback: Callback<Result<LightCollection, Error>>,
    task: Option<FetchTask>,
    console: ConsoleService,
}

pub enum Msg {
    GetAllLights,
    AllLights(Result<LightCollection, Error>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        App {
            lights: None,
            api: HooApi::new("http://localhost:8000"),
            lights_callback: link.send_back(Msg::AllLights),
            task: None,
            console: ConsoleService::new(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.update(Msg::GetAllLights);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetAllLights => {
                self.task = Some(self.api.get_all_lights(self.lights_callback.clone()));
                false
            }
            Msg::AllLights(lights) => {
                match lights {
                    Ok(lights) => {
                        for light in &lights {
                            self.console.log(&format!("{}", light.0));
                        }
                        self.lights = Some(lights);
                        true
                    },
                    Err(err) => {
                        self.console.log(&format!("{}", err));
                        false
                    },
                }
            }
        }
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::GetAllLights>{ "Get lights" }</button>
                <p>{ "Hello world!" }</p>
            </div>
        }
    }
}
