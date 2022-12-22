pub mod color;
pub mod connection;
pub mod error;
pub mod light;
pub mod motion;

pub use crate::color::Color;
pub use crate::connection::ApiConnection;
pub use crate::error::HooApiError;
pub use crate::light::{Light, LightCollection, LightNumber, LightState};
pub use crate::motion::Motion;