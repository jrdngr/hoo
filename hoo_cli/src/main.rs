use structopt::StructOpt;
use hoo_api::{ApiConnection, LightState};

mod options;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = options::Options::from_args();
    
    let connection = ApiConnection::new(&options.hue_base_uri, &options.hue_user_id);

    use options::Command::*;
    match options.command {
        On{ light_num } => { connection.on(light_num).await?; },
        Off{ light_num } => { connection.off(light_num).await?; },
        Toggle{ light_num } => {
            let light = connection.get_light(light_num).await?;
            let new_state = match light.state.on {
                Some(is_on) => LightState::new().on(!is_on),
                None => LightState::new().on(true),
            };
            connection.set_state(light_num, &new_state).await?;
        },
        TransitionTime{ light_num, value } => { connection.transition_time(light_num, value).await?; },
        Red{ light_num, value } => todo!(),
        Green{ light_num, value } => todo!(),
        Blue{ light_num, value } => todo!(),
        Hue { light_num, value } => { connection.set_state(light_num, &LightState::new().hue(value)).await?; },
        Saturation{ light_num, value } => { connection.set_state(light_num, &LightState::new().sat(value)).await?; },
        Brightness{ light_num, value } => { connection.set_state(light_num, &LightState::new().bri(value)).await?; },
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
