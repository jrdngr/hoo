use std::str::FromStr;

use anyhow::Result;
use hyper::client::{Client, HttpConnector};
use hyper::{Body, Request, Response, Uri};

#[derive(Debug, Clone)]
pub struct HueClient {
    client: Client<HttpConnector>,
    base_uri: String,
}

impl HueClient {
    pub fn new(hue_base_uri: &str, hue_user_id: &str) -> Self {
        let base_uri = format!("{}/api/{}", hue_base_uri, hue_user_id);
        Self {
            client: Client::new(),
            base_uri: base_uri.to_string(),
        }
    }

    pub async fn get(&self, endpoint: &str) -> Result<Response<Body>> {
        let uri = Uri::from_str(&format!("{}/{}", self.base_uri, endpoint))?;
        Ok(self.client.get(uri).await?)
    }

    pub async fn handle(&self, request: Request<Body>) -> Result<Response<Body>> {
        Ok(self.client.request(request).await?)
    }
}
