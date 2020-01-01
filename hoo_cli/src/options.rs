use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(env, hide_env_values = true)]
    pub hue_base_uri: String,
    #[structopt(env, hide_env_values = true)]
    pub hue_user_id: String,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    On{ light_num: u8 },
    Off{ light_num: u8 },
    Toggle{ light_num: u8 },
    TransitionTime{ light_num: u8, value: u16 },
    Red{ light_num: u8, value: u8 },
    Green{ light_num: u8, value: u8 },
    Blue{ light_num: u8, value: u8 },
    Hue { light_num: u8, value: u16 },
    Saturation{ light_num: u8, value: u8 },
    Brightness{ light_num: u8, value: u8 },
    List {
        #[structopt(long)]
        active: bool,
        #[structopt(long, short = "n")]
        light_num: Option<u8>,        
    },
}
