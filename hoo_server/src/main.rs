use structopt::StructOpt;

use std::io::{Result, Error, ErrorKind};

use hoo_base::{Hoo, HooConfig};

pub use server::HooServer;

pub mod options;
pub mod server;

#[actix_rt::main]
async fn main() -> Result<()> {
    let options = options::Options::from_args();

    if options.create_config {
        write_default_config_file()?;
        return Ok(());
    }

    let (hoo, sender) = if let Some(config_file) = options.config_file {
        Hoo::with_config_file(config_file)
            .map_err(|e| Error::new(ErrorKind::Other, e))?
    } else {
        Hoo::new()
    };

    let config = hoo.config().clone();

    // std::thread::spawn(move || hoo.run());

    HooServer::run(&config, sender).await
}

fn write_default_config_file() -> Result<()> {
    let config = HooConfig::default();
    config.write_default_file()
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    Ok(())
}
