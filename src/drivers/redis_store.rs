use crate::datastore::Datastore;
use crate::error::LucyError;
use crate::record::Record;

pub struct RedisStore {
    con: redis::Connection,
}

impl RedisStore {
    pub fn new() -> RedisStore {
        // @TODO take connection string as parameter.
        let client = redis::Client::open("redis://127.0.0.1/").expect("could not connect to redis");

        let con = client
            .get_connection()
            .expect("could not get connection to redis");

        RedisStore { con }
    }
}

impl Datastore for RedisStore {
    fn find(&mut self, uuid: &str) -> Result<Record, LucyError> {
        match redis::cmd("GET").arg(uuid).query::<String>(&mut self.con) {
            Ok(url) => {
                match Record::from(url, uuid.to_string()) {
                    Ok(record) => Ok(record),
                    Err(_) => Err(LucyError::NotAValidUrlError)
                }
            },
            Err(_) => Err(LucyError::UrlNotFoundError),
        }
    }

    fn record(&mut self, record: Record) -> Result<bool, String> {
        match redis::cmd("SET").arg(record.uuid).arg(record.url).query::<String>(&mut self.con) {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }

    fn all(&self) -> Vec<Record> {
        vec![]
    }
}
