use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "options")]
pub struct Options {
    /// Loads the given config file and runs the server.
    #[structopt(short = "c", long)]
    pub config_file: Option<String>,

    /// Creates a default config file. Does not run the server.
    #[structopt(long, raw(conflicts_with = "\"config_file\""))]
    pub create_config: bool,

    // Runs the server using a fake api with state stored in the given file.
    #[structopt(long, short, parse(from_os_str))]
    pub from_file: Option<PathBuf>,
}
