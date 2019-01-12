use std::io::stdin;
use std::str::FromStr;
use std::time::Duration;

use clap::{Arg, App, SubCommand, values_t};

use hoo::effects;
use hoo::AnyError;
use hoo::{Hoo, HooCommand};

use hoohue_api::color::Color;
use hoohue_api::light::LightState;
use hoohue_api::ApiConnection;

fn main() -> Result<(), AnyError> {
    dotenv::dotenv().ok();

    let base_uri = std::env::var("HUE_BASE_URI").expect("HUE_BASE_URI must be set");
    let user_id = std::env::var("HUE_USER_ID").expect("HUE_USER_ID must be set");

    let (hoo, sender) = Hoo::new();
    std::thread::spawn(move || hoo.run());

    let matches = App::new("Hoo")
                    .version("0.1")
                    .author("Jordan G. <jordangrapes@gmail.com>")
                    .about("Fancy pants controls for your Hue lights")
                    .subcommand(SubCommand::with_name("on")
                        .about("Turns lights on")
                        .arg(Arg::with_name("lights")
                            .long("lights")
                            .short("l")
                            .help("The light numbers to turn on. Applies to all lights if this is not specified")
                            .required(false)
                            .takes_value(true)
                            .multiple(true)))
                    .get_matches();

    match matches.subcommand() {
        ("on", Some(on_matches)) => {
            let lights_typed = values_t!(on_matches, "lights", u8).unwrap_or(Vec::new());
            
            for light in &lights_typed {
                // I think the program ends before this can be processed
                sender.send(HooCommand::On(*light));
            }
        },
        _ => unreachable!(),
    }

    Ok(())
}