pub use hoo_api_types::{Color, Light, LightCollection, LightNumber, LightState};

use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use hyper::client::HttpConnector;
use hyper::{body, Body, Request, Response, Uri};

use hoo_api_types::LightEffect;

#[derive(Debug, Clone)]
pub struct HueClient {
    pub client: hyper::Client<HttpConnector>,
    base_uri: String,
}

impl HueClient {
    pub fn new(base_uri: &str, user_id: &str) -> Self {
        let base_uri = format!("{}/api/{}", base_uri, user_id);
        Self {
            client: hyper::Client::new(),
            base_uri: base_uri.to_string(),
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Response<Body>> {
        let uri = Uri::from_str(&format!("{}/{}", self.base_uri, endpoint))?;
        Ok(self.client.get(uri).await?)
    }

    pub async fn put<T>(&self, endpoint: &str, body: T) -> Result<Response<Body>> 
    where T: Into<Body>
    {
        let uri = Uri::from_str(&format!("{}/{}", self.base_uri, endpoint))?;
        let request = Request::builder()
            .method("PUT")
            .uri(uri)
            .body(body.into())
            .unwrap();

        self.handle(request).await
    }

    pub async fn handle(&self, request: Request<Body>) -> Result<Response<Body>> {
        Ok(self.client.request(request).await?)
    }

    pub async fn get_all_lights_response(&self) -> Result<Response<Body>> {
        self.get("lights").await
    }

    pub async fn get_all_lights(&self) -> Result<LightCollection> {
        let response = self.get_all_lights_response().await?;
        let lights: HashMap<u8, Light> = deserialize_response(response).await?;
        Ok(lights)
    }

    pub async fn get_active_lights(&self) -> Result<LightCollection> {
        let active_lights = self
            .get_all_lights()
            .await?
            .into_iter()
            .filter(|(_, l)| l.state.is_on() && l.state.is_reachable())
            .collect();

        Ok(active_lights)
    }

    pub async fn get_light_response(&self, light_number: u8) -> Result<Response<Body>> {
        let uri = format!("lights/{}", light_number);
        self.get(&uri).await
    }

    pub async fn get_light(&self, light_number: u8) -> Result<Light> {
        let response = self.get_light_response(light_number).await?;
        deserialize_response(response).await
    }

    pub async fn set_state(&self, light_number: u8, state: &LightState) -> Result<Response<Body>> {
        let body = serde_json::to_string(state)?;
        let uri = format!("lights/{}/state", light_number);
        self.put(&uri, body).await
    }

    pub async fn on(&self, light_number: u8) -> Result<Response<Body>> {
        let state = LightState::new().on(true);
        self.set_state(light_number, &state).await
    }

    pub async fn off(&self, light_number: u8) -> Result<Response<Body>> {
        let state = LightState::new().on(false);
        self.set_state(light_number, &state).await
    }

    pub async fn colorloop(&self, light_number: u8, enabled: bool) -> Result<Response<Body>> {
        let effect = if enabled {
            LightEffect::ColorLoop
        } else {
            LightEffect::None
        };
        let state = LightState::new().effect(effect);
        self.set_state(light_number, &state).await
    }

    pub async fn transition_time(
        &self,
        light_number: u8,
        transition_time: u16,
    ) -> Result<Response<Body>> {
        let state = LightState::new().transitiontime(transition_time);
        self.set_state(light_number, &state).await
    }
}

pub async fn response_to_string(response: Response<Body>) -> Result<String> {
    let body_bytes = body::to_bytes(response.into_body()).await?.to_vec();
    Ok(String::from_utf8(body_bytes)?)
}

pub async fn deserialize_response<T>(response: Response<Body>) -> Result<T> 
where T: serde::de::DeserializeOwned,
{
    let body_bytes = body::to_bytes(response.into_body()).await?;
    let result = serde_json::from_slice(&body_bytes)?;
    Ok(result)
}
