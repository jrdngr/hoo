use failure::Fail;
use std::convert::From;

#[derive(Debug, Fail)]
pub enum HooApiError {
    #[fail(display = "Connection error: {}", _0)]
    ConnectionError(String),
    #[fail(display = "Other error: {}", _0)]
    OtherError(#[cause] failure::Error),
}

impl From<failure::Error> for HooApiError {
    fn from(error: failure::Error) -> Self {
        HooApiError::OtherError(error)
    }
}
