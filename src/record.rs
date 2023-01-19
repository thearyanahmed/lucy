use url::{Url, ParseError};
use rand::{distributions::Alphanumeric, Rng};

pub struct Record {
    pub url: String,
    pub uuid: String,
}

impl Record {
    pub fn from(url: String, uuid: String) -> Result<Self,ParseError> {
        match Url::parse(&url) {
            Ok(parsed_url) => Ok(Record{ url: parsed_url.to_string(), uuid }),
            Err(err) => Err(err),
        }
    }

    pub fn new(url: String) -> Self {
        let uuid = Record::generate_uuid(6);

        Record { url, uuid }
    }

    pub fn using_uuid(&mut self, uuid: String) {
        self.uuid = uuid;
    }

    fn generate_uuid(char_length : usize) -> String {
        let unique_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(char_length)
        .map(char::from)
        .collect();

        unique_string
    }
}