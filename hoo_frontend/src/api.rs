use failure::{format_err, Error};
use yew::callback::Callback;
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::services::ConsoleService;
use yew::format::{Json, Nothing};
use hoo_api_types::LightCollection;   

pub struct HooApi {
    base_url: String,
    console: ConsoleService,
    fetch_service: FetchService,
}

impl HooApi {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            console: ConsoleService::new(),
            fetch_service: FetchService:: new(),
        }
    }

    pub fn get_all_lights(&mut self, callback: Callback<Result<LightCollection, Error>>) -> FetchTask {
        let url = format!("{}/api/lights", self.base_url);
        let handler = move |response: Response<Json<Result<LightCollection, Error>>>| {
            let (meta, Json(data)) = response.into_parts();

            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(format_err!("{}: Error getting lights", meta.status)))
            }
        };

        let request = Request::get(url)
            .body(Nothing)
            .expect("Failed to build request");

        self.fetch_service.fetch(request, handler.into())
    }
}

// export async function getAllLights(): Promise<HooLight[]> {
//     const url = `${BASE_URL}/lights`;
//     const response = await fetch(url);
//     const lights: HooLight[] = await response.json();
//     return lights;
// }

// export async function getLight(lightNumber: number): Promise<HooLight> {
//     const url = `${BASE_URL}/light/${lightNumber}`;
//     const response: any = await fetch(url);
//     const light: HooLight = await response.json();
//     return light;
// }

// export async function on(lightNumber: number) {
//     const url = `${BASE_URL}/${lightNumber}/on`;
//     await fetch(url);
// }

// export async function off(lightNumber: number) {
//     const url = `${BASE_URL}/${lightNumber}/off`;
//     await fetch(url);
// }

// export async function setBrightness(lightNumber: number, brightness: number) {
//     const url = `${BASE_URL}/${lightNumber}/state?bri=${brightness}`;
//     await fetch(url);
// }

// export async function setSaturation(lightNumber: number, saturation: number) {
//     const url = `${BASE_URL}/${lightNumber}/state?sat=${saturation}`;
//     await fetch(url);
// }

// export async function setHue(lightNumber: number, hue: number) {
//     const url = `${BASE_URL}/${lightNumber}/state?hue=${hue}`;
//     await fetch(url);
// }

// import { BASE_URL } from '@/common/constants';

// export async function rotate(transitionTime: number, holdTime: number): Promise<void> {
//     const url = `${BASE_URL}/rotate/${transitionTime}/${holdTime}`;
//     await fetch(url);
// }

// export async function random(transitionTime: number, holdTime: number): Promise<void> {
//     const url = `${BASE_URL}/random/${transitionTime}/${holdTime}`;
//     await fetch(url);
// }

// export async function sleepy(transitionTime: number, holdTime: number): Promise<void> {
//     const url = `${BASE_URL}/sleepy/${transitionTime}/${holdTime}`;
//     await fetch(url);
// }

// export async function stop(): Promise<void> {
//     const url = `${BASE_URL}/stop`;
//     await fetch(url);
// }