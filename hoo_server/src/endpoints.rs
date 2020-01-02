use anyhow::Result;
use hyper::{Body,Response};

use crate::HueClient;

pub async fn get_all_lights(client: &HueClient) -> Result<Response<Body>> {
    client.get("lights").await
}
