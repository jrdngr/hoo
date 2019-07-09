use actix_web::Result;

use std::thread;

use hoo_base::{Hoo, HooConfig};
pub use server::HooServer;

pub mod server;

fn main() -> Result<()> {
    let config = HooConfig::from_dotenv();

    let (hoo, sender) = Hoo::new();

    thread::spawn(move || hoo.run());

    HooServer::run(&config, sender)?;

    Ok(())
}
