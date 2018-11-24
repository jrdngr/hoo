type AnyError = Box<dyn std::error::Error>;

use hoo::api::ApiConnection;

fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let connection = ApiConnection::new(&base_uri, &user_id);


    Ok(())
}