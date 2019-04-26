use clap::{values_t, App, Arg, SubCommand};

use hoo::{Hoo, HooCommand};

fn main() -> Result<(), failure::Error> {
    dotenv::dotenv().ok();

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
            let lights_typed = values_t!(on_matches, "lights", u8).unwrap_or_default();

            for light in &lights_typed {
                // I think the program ends before this can be processed
                let _ = sender.send(HooCommand::On(*light));
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
