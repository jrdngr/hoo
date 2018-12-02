use hoohue_api::ApiConnection;

pub mod animation;
pub mod effects;

pub type AnyError = Box<dyn std::error::Error>;

pub struct Hoo {
    connection: ApiConnection,
}
