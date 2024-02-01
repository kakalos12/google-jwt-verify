#[cfg(test)]
mod test;

mod algorithm;
mod client;
mod error;
mod header;
mod jwk;
mod key_provider;
mod token;
mod unverified_token;

pub use crate::client::Client;
pub use crate::token::{IdPayload, RequiredClaims, Token};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

pub use error::Error;

fn base64_decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE.decode(input)
}
