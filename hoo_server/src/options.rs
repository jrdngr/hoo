use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(env, hide_env_values = true)]
    pub hue_base_uri: String,
    #[structopt(env, hide_env_values = true)]
    pub hue_user_id: String,
}
