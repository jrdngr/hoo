use structopt::StructOpt;
use hoo_api::{ApiConnection, Color, LightState};

mod options;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = options::Options::from_args();
    
    let connection = ApiConnection::new(&options.hue_base_uri, &options.hue_user_id);

    use options::Command::*;
    match options.command {
        On { light_num } => { connection.on(light_num).await?; },
        Off { light_num } => { connection.off(light_num).await?; },
        Toggle { light_num } => {
            let light = connection.get_light(light_num).await?;
            let new_state = match light.state.on {
                Some(is_on) => LightState::new().on(!is_on),
                None => LightState::new().on(true),
            };
            connection.set_state(light_num, &new_state).await?;
        },
        TransitionTime { light_num, value } => { connection.transition_time(light_num, value).await?; },
        Red { light_num, value } => {
            let (_, g, b) = connection.get_light(light_num).await?.color().unwrap_or_default().rgb();
            let new_state = LightState::new()
                .color(&Color::from_rgb(value, g, b))
                .sat(255);
            connection.set_state(light_num, &new_state).await?;
        },
        Green { light_num, value } => {
            let (r, _, b) = connection.get_light(light_num).await?.color().unwrap_or_default().rgb();
            let new_state = LightState::new()
                .color(&Color::from_rgb(r, value, b))
                .sat(255);
            connection.set_state(light_num, &new_state).await?;
        },
        Blue { light_num, value } => {
            let (r, g, _) = connection.get_light(light_num).await?.color().unwrap_or_default().rgb();
            let new_state = LightState::new()
                .color(&Color::from_rgb(r, g, value))
                .sat(255);
            connection.set_state(light_num, &new_state).await?;
        },
        RGB { light_num, red, green, blue } => {
            let new_state = LightState::new()
                .color(&Color::from_rgb(red, green, blue))
                .sat(255);
            connection.set_state(light_num, &new_state).await?;
        },
        Hue { light_num, value } => { connection.set_state(light_num, &LightState::new().hue(value)).await?; },
        Sat { light_num, value } => { connection.set_state(light_num, &LightState::new().sat(value)).await?; },
        Bri { light_num, value } => { connection.set_state(light_num, &LightState::new().bri(value)).await?; },
        HSB { light_num, hue, sat, bri } => {
            let new_state = LightState::new().color(&Color::from_hsv(hue, sat, bri));
            connection.set_state(light_num, &new_state).await?;
        },
        List { active, light_num } => {
            if let Some(light_num) = light_num {
                let light = connection.get_light(light_num).await?;
                dbg!(light);
            } else if active {
                let lights = connection.get_active_lights().await?;
                dbg!(lights);
            } else {
                let lights = connection.get_all_lights().await?;
                dbg!(lights);
            }
        },
    };

    Ok(())
}
