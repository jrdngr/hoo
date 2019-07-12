use actix_web::Result;
use structopt::StructOpt;

use std::error::Error;
use std::thread;

use hoo_base::{Hoo, HooConfig};

pub use server::HooServer;

pub mod options;
pub mod server;

fn main() -> Result<(), Box<Error>> {
    let options = options::Options::from_args();

    if options.create_config {
        write_default_config_file()?;
        return Ok(());
    }

    let (hoo, sender) = if let Some(config_file) = options.config_file {
        Hoo::with_config_file(config_file)?
    } else {
        Hoo::new()
    };
    let config = hoo.config().clone();

    thread::spawn(move || hoo.run());

    HooServer::run(&config, sender)?;

    Ok(())
}

fn write_default_config_file() -> Result<(), Box<Error>> {
    let config = HooConfig::default();
    config.write_default_file()?;
    Ok(())
}