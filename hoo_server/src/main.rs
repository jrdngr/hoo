use actix_web::Result;

use std::thread;

use hoo_base::Hoo;
pub use server::HooServer;

pub mod server;

fn main() -> Result<()> {
    let (hoo, sender) = Hoo::new();
    let config = hoo.config().clone();

    thread::spawn(move || hoo.run());

    HooServer::run(&config, sender)?;

    Ok(())
}
