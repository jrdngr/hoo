mod options;

use anyhow::Result;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use structopt::StructOpt;
use hoo_api::HueClient;

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
    let mut path = req.uri()
        .path()
        .trim_start_matches('/')
        .split('/')
        .into_iter();
    
    match path.next() {
        Some("api") => handle_api(req.method(), path, client).await,
        _ => Ok(not_found())
    }
}

async fn handle_api(method: &Method, mut path: impl Iterator<Item = &str>, client: HueClient) -> Result<Response<Body>> {
    match path.next() {
        None => Ok(not_found()),
        Some(endpoint) => match (method, endpoint) {
            (&Method::GET, "lights") => client.get_all_lights_response().await,
            (&Method::GET, "light") => {
                match path.next() {
                    Some(light_num) => handle_light_state(method, light_num, path, client).await,
                    None => Ok(not_found()),
                }
            },
            _ => Ok(not_found())
        }
    }
}

async fn handle_light_state(method: &Method, light_num: &str, mut path: impl Iterator<Item = &str>, client: HueClient) -> Result<Response<Body>> {
    let light_num: u8 = light_num.parse().expect("Invalid light number");
    match path.next() {
        None => client.get_light_response(light_num).await,
        Some(command) => match (method, command) {
            (&Method::GET/*PUT*/, "on") => client.on(light_num).await,
            (&Method::GET/*PUT*/, "off") => client.off(light_num).await,
            _ => Ok(not_found())
        }
    }
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}
