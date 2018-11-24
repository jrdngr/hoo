type AnyError = Box<dyn std::error::Error>;

use hoo::api::{ApiConnection, Light};

fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let connection = ApiConnection::new(&base_uri, &user_id);

    let uri = format!("{}/{}/lights/1", base_uri, user_id);

    let response = connection.client.get(uri.as_str())
        .send()?
        .text()?;

    let light: Light = serde_json::from_str(&response)?;

    println!("{:?}", light);

    Ok(())
}