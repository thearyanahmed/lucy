pub mod datastore;
pub mod drivers;
pub mod error;

use url::{Url, ParseError};
use rand::{distributions::Alphanumeric, Rng};

pub struct Record {
    pub url: String,
    pub uuid: String,
}
