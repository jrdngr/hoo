mod options;

use anyhow::{anyhow, Result};
use structopt::StructOpt;
use warp::Filter;

use hoo_api::HueClient;
use hoo_api_types::LightState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let options = options::Options::from_args();

    let addr = "127.0.0.1:8000".parse().unwrap();

    let client = HueClient::new(&options.hue_base_uri, &options.hue_user_id);
    println!("Hoo server listening on http://{}", addr);

    let preflight = warp::options().map(|| preflight());
    let all_lights = warp::get().and(warp::path("lights").map(|| client.get_all_lights_response()));
    
    let light =
        warp::get().and(warp::path!("light" / u8).map(|light_num| client.get_light_response(light_num)))
        .or(
            warp::put().and(
                warp::path!(u8/ "on").map(|light_num| client.on(light_num))
                .or(warp::path!(u8 / "off").map(|light_num| client.off(light_num)))
                .or(warp::path!(u8 / "toggle").map(|light_num| client.toggle(light_num)))
                .or(warp::path!(u8 / "state").and(warp::body::json()).map(|light_num, state: LightState| client.set_state(light_num, &state)))
            )
    );

    let routes = warp::path("api")
        .and(
            preflight
            // .or(all_lights)
            // .or(light)
        );
    
    warp::serve(routes).run(addr).await;

    Ok(())
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
