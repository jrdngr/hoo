use serde_derive::Deserialize;
use reqwest::Client;

use crate::AnyError;

pub struct ApiConnection {
    pub client: reqwest::Client,
    base_uri: String,
}

impl ApiConnection {
    pub fn new(base_uri: &str, user_id: &str) -> Self {
        let base_uri = format!("{}/api/{}", base_uri, user_id);
        Self { 
            client: reqwest::Client::new(),
            base_uri: base_uri.to_string(),
        }
    }

    pub fn base(&self) -> String {
        self.base_uri.clone()
    }
}


pub fn on(connection: &ApiConnection, light_number: u8) -> Result<String, AnyError> {
    let body = r#"{"on": true}"#;

    let uri = format!("{}/lights/{}/state", connection.base(), light_number);

    println!("{}", uri);

    let response = connection.client.put(&uri)
        .body(body)
        .send()?
        .text()?;

        println!("{}", &response);

    Ok(response)
}

pub fn off(connection: &ApiConnection, light_number: u8) -> Result<String, AnyError> {
    let body = r#"{"on": false}"#;

    let uri = format!("{}/lights/{}/state", connection.base(), light_number);

    let response = connection.client.put(&uri)
        .body(body)
        .send()?
        .text()?;

    Ok(response)
}