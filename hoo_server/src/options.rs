use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "options")]
pub struct Options {
    /// Loads the given config file and runs the server.
    #[structopt(short = "c", long)]
    pub config_file: Option<String>,

    /// Creates a default config file. Does not run the server.
    #[structopt(long, conflicts_with = "config_file")]
    pub create_config: bool,
}
