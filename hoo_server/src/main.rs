mod options;
mod static_files;

use anyhow::{anyhow, Result};
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
    let path = req.uri().path().to_string();
    println!("Routing: {}", path);

    let mut path = path
        .trim_start_matches('/')
        .split('/')
        .into_iter();
    
    let result = match path.next() {
        Some("api") => handle_api(req, path, client).await,
        Some("index.html") | Some("") => Ok(static_files::index()),
        Some("pkg") => {
            match path.next() {
                Some("package.js") => Ok(static_files::package_js()),
                Some("package_bg.wasm") => Ok(static_files::package_wasm()),
                _ => Ok(not_found()),
            }
        },
        _ => Ok(not_found()),
    };

    match result {
        Ok(response) => Ok(response),
        Err(e) => { 
            dbg!(e);
            Ok(internal_server_error())
        },
    }
}

async fn handle_api(req: Request<Body>, mut path: impl Iterator<Item = &str>, client: HueClient) -> Result<Response<Body>> {
    match path.next() {
        None => Ok(not_found()),
        Some(endpoint) => match (req.method(), endpoint) {
            (&Method::OPTIONS, _) => Ok(preflight()),
            (&Method::GET, "lights") => client.get_all_lights_response().await,
            (&Method::GET, "light") =>  { 
                let light_num: u8 = path.next()
                    .ok_or(anyhow!("Missing light number"))?
                    .parse()?;
                client.get_light_response(light_num).await 
            },
            (&Method::PUT, "light") =>  handle_light_state(req, path, client).await,
            _ => Ok(not_found())
        }
    }
}

async fn handle_light_state(req: Request<Body>, mut path: impl Iterator<Item = &str>, client: HueClient) -> Result<Response<Body>> {
    let light_num: u8 = path.next()
        .ok_or(anyhow!("Missing light number"))?
        .parse()?;

    match path.next() {
        Some(command) => match (req.method(), command) {
            (&Method::PUT, "on") => client.on(light_num).await,
            (&Method::PUT, "off") => client.off(light_num).await,
            (&Method::PUT, "toggle") => client.toggle(light_num).await,
            (&Method::PUT, "state") => client.set_state_from_body(light_num, req.into_body()).await,
            _ => Ok(not_found())
        }
        _ => Ok(not_found())
    }
}

fn preflight() -> Response<Body> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "*")
        .status(StatusCode::OK)
        .body("".into())
        .unwrap()
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not Found".into())
        .unwrap()
}

fn internal_server_error() -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Internal Server Error".into())
        .unwrap()
}
