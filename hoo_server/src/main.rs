mod endpoints;
mod options;
mod utils;

use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use structopt::StructOpt;

use hoo_hue_client::HueClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let options = options::Options::from_args();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let client = HueClient::new(&options.hue_base_uri, &options.hue_user_id);

    let new_service = make_service_fn(move |_| {
        let client = client.clone();
        async {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                handle(req, client.to_owned())
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);

    println!("Hoo server listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn handle(req: Request<Body>, client: HueClient) -> Result<Response<Body>> {
    use utils::next_path_component;

    let path = req.uri().path();

    match next_path_component(path) {
        Some(("api", endpoint)) => handle_api(req.method(), endpoint, client).await,
        _ => Ok(not_found())
    }
}

async fn handle_api(method: &Method, endpoint: &str, client: HueClient) -> Result<Response<Body>> {
    match (method, endpoint) {
        (&Method::GET, "lights") => endpoints::get_all_lights(&client).await,
        _ => Ok(not_found())
    }
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}
