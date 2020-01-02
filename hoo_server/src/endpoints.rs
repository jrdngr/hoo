use anyhow::Result;
use hyper::{Body,Response};

use crate::HueClient;

pub async fn get_all_lights(client: &HueClient) -> Result<Response<Body>> {
    client.get("lights").await
}

pub async fn get_light(client: &HueClient, light_num: u8) -> Result<Response<Body>> {
    let uri = format!("lights/{}", light_num);
    client.get(&uri).await
}
