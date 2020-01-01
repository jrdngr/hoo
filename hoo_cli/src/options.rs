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
    On { light_num: u8 },
    Off { light_num: u8 },
    Toggle { light_num: u8 },
    TransitionTime { light_num: u8, value: u16 },
    Red { light_num: u8, value: f64 },
    Green { light_num: u8, value: f64 },
    Blue { light_num: u8, value: f64 },
    RGB { light_num: u8, red: f64, green: f64, blue: f64},
    Hue { light_num: u8, value: u16 },
    Sat { light_num: u8, value: u8 },
    Bri { light_num: u8, value: u8 },
    HSB { light_num: u8, hue: u16, sat: u8, bri: u8 },
    List {
        light_num: Option<u8>,
        #[structopt(long)]
        active: bool,     
    },
}
