use structopt::StructOpt;
use hoo_api::ApiConnection;

mod options;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = options::Options::from_args();
    
    let connection = ApiConnection::new(&options.hue_base_uri, &options.hue_user_id);

    use options::Command::*;
    match options.command {
        On{ light_num } => { connection.on(light_num).await?; },
        Off{ light_num } => { connection.off(light_num).await?; },
        Toggle{ light_num } => todo!(),
        TransitionTime{ light_num, value } => todo!(),
        Red{ light_num, value } => todo!(),
        Green{ light_num, value } => todo!(),
        Blue{ light_num, value } => todo!(),
        Hue { light_num, value } => todo!(),
        Saturation{ light_num, value } => todo!(),
        Brightness{ light_num, value } => todo!(),
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
