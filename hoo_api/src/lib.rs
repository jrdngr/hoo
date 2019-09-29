pub mod connection;
pub mod error;

pub use crate::connection::ApiConnection;
pub use crate::error::HooApiError;
pub use hoo_api_types::{Color, Light, LightCollection, LightNumber, LightState};
