mod options;

use std::convert::Infallible;
use anyhow::Result;
use structopt::StructOpt;
use warp::Filter;

use hoo_api::HueClient;
use hoo_api_types::LightState;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let options = options::Options::from_args();

    let addr: std::net::SocketAddr = "127.0.0.1:8000".parse().unwrap();

    let client = HueClient::new(&options.hue_base_uri, &options.hue_user_id);
    println!("Hoo server listening on http://{}", addr);

    let all_lights = warp::get().and(warp::path("lights").and_then(|| get_all_lights(&client)));
    
    let light = warp::path!("light" / u8);
    
    let get_light = warp::get().and(light).and_then(|light_num| get_light(&client, light_num));

    let light_on = light.and(warp::path("on"))
        .and_then(|light_num| on(&client, light_num));

    let light_off = light.and(warp::path("off"))
        .and_then(|light_num| off(&client, light_num));

    let light_toggle = light.and(warp::path("toggle"))
        .and_then(|light_num| toggle(&client, light_num));
    
    let light_state = light.and(warp::path("state"))
        .and(warp::body::json())
        .and_then(|light_num, state| set_state(&client, light_num, state));

    let put_light = warp::put().and(
        light_on
        .or(light_off)
        .or(light_toggle)
        .or(light_state)
    );

    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET", "PUT"]);

    let routes = warp::path("api")
        .and(
            all_lights
            .or(get_light)
            .or(put_light)
        )
        .with(cors);
    
    warp::serve(routes).run(addr).await;

    Ok(())
}

async fn get_all_lights(client: &HueClient) -> Result<impl warp::Reply, Infallible> {
    match client.get_all_lights().await {
        Ok(lights) => Ok(warp::reply::json(&lights)),
        Err(e) => Ok(warp::reply::json(&format!("{}", e))),
    }
}

async fn get_light(client: &HueClient, light_num: u8) -> Result<impl warp::Reply, Infallible> {
    match client.get_light(light_num).await {
        Ok(light) => Ok(warp::reply::json(&light)),
        Err(e) => Ok(warp::reply::json(&format!("{}", e))),
    }
}

async fn on(client: &HueClient, light_num: u8) -> Result<impl warp::Reply, Infallible> {
    match client.on(light_num).await {
        Ok(_) => Ok(warp::reply::json(&format!("Light {} turned on", light_num))),
        Err(e) => Ok(warp::reply::json(&format!("{}", e))),
    }
}

async fn off(client: &HueClient, light_num: u8) -> Result<impl warp::Reply, Infallible> {
    match client.off(light_num).await {
        Ok(_) => Ok(warp::reply::json(&format!("Light {} turned off", light_num))),
        Err(e) => Ok(warp::reply::json(&format!("{}", e))),
    }
}

async fn toggle(client: &HueClient, light_num: u8) -> Result<impl warp::Reply, Infallible> {
    match client.toggle(light_num).await {
        Ok(_) => Ok(warp::reply::json(&format!("Light {} toggled", light_num))),
        Err(e) => Ok(warp::reply::json(&format!("{}", e))),
    }
}

async fn set_state(client: &HueClient, light_num: u8, state: LightState) -> Result<impl warp::Reply, Infallible> {
    match client.set_state(light_num, &state).await {
        Ok(_) => Ok(warp::reply::json(&format!("Light {} state set to\n{:?}", light_num, &state))),
        Err(e) => Ok(warp::reply::json(&format!("{}", e))),
    }
}
