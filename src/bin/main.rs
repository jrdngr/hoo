use std::io::{stdin, Read};

use hoo::AnyError;
use hoo::light::{Light};
use hoo::api;
use hoo::api::{ApiConnection};


fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let connection = hoo::api::ApiConnection::new(&base_uri, &user_id);
 
    let mut buffer = String::new();

    loop {
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Failed to read line");
        buffer = buffer.trim().to_string();

        if buffer == "quit".to_string() {
            println!("Bye!");
            break;
        } else if buffer == "on".to_string() {
            api::on(&connection, 1);
        } else if buffer == "off".to_string() {
            api::off(&connection, 1);
        } else {
            println!("Unknown command");
        }
    }

    Ok(())
}