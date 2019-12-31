use structopt::StructOpt;

use std::io::{Result, Error, ErrorKind};

use hoo_base::{Hoo, HooConfig};

// pub use server::HooServer;

pub mod options;
// pub mod server;

#[tokio::main]
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

    tokio::spawn(async move { 
        hoo.run().await
    });

    // HooServer::run(&config, sender).await

    Ok(())
}

fn write_default_config_file() -> Result<()> {
    let config = HooConfig::default();
    config.write_default_file()
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    Ok(())
}
