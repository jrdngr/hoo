use structopt::StructOpt;

use std::io::{Result, Error, ErrorKind};

use hoo_base::{Hoo, HooConfig};
use std::path::PathBuf;
use std::thread;

pub use server::HooServer;

pub mod options;
pub mod server;

#[actix_rt::main]
async fn main() -> Result<()> {
    run()
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e))
}

async fn run() -> anyhow::Result<()> {
    let options = options::Options::from_args();

    if let Some(file_path) = options.from_file {
        run_test_server(file_path).await
    } else {
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

        HooServer::run(&config, sender).await
    }
}

async fn run_test_server(file_path: PathBuf) -> anyhow::Result<()> {
    let (hoo, sender) = Hoo::from_file(file_path);
    let config = hoo.config().clone();

    thread::spawn(move || hoo.run());
    HooServer::run(&config, sender).await;

    Ok(())
}

fn write_default_config_file() -> anyhow::Result<()> {
    let config = HooConfig::default();
    config.write_default_file()?;
    Ok(())
}
