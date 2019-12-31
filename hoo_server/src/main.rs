use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use structopt::StructOpt;

use std::net::SocketAddr;
use std::convert::Infallible;
use std::io::{Result, Error, ErrorKind};

use hoo_base::{Hoo, HooConfig};

pub mod options;
pub mod service;

#[tokio::main]
async fn main() -> Result<()> {
    let options = options::Options::from_args();

    if options.create_config {
        write_default_config_file()?;
        return Ok(());
    }

    let (mut hoo, sender) = if let Some(config_file) = options.config_file {
        Hoo::with_config_file(config_file)
            .map_err(|e| Error::new(ErrorKind::Other, e))?
    } else {
        Hoo::new()
    };

    let config = hoo.config().clone();

    tokio::spawn(async move { 
        hoo.run().await
    });

    let addr: SocketAddr = config
        .hoo_server_socket_uri
        .parse()
        .expect(&format!("Invalid socket URI: {}", config.hoo_server_socket_uri));

    println!("Running Hoo server at: {}", config.hoo_server_socket_uri);

    let service = service::HooService::new(sender);

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}

fn write_default_config_file() -> Result<()> {
    let config = HooConfig::default();
    config.write_default_file()
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    Ok(())
}
