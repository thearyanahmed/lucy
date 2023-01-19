use crate::datastore::Datastore;
use crate::error::LucyError;
use crate::record::Record;
use std::collections::HashMap;
use std::vec;

pub struct HashmapStore {
    store: HashMap<String, String>,
}

impl HashmapStore {
    pub fn new() -> HashmapStore {
        let store: HashMap<String, String> = HashMap::new();
        HashmapStore { store }
    }
}

impl Datastore for HashmapStore {
    fn find(&mut self, uuid: &str) -> Result<Record, LucyError> {
        match self.store.get(uuid) {
            Some(url) => match Record::from(url.to_string(), uuid.to_string()) {
                Ok(rec) => Ok(rec),
                Err(_) => Err(LucyError::NotAValidUrlError),
            },
            None => Err(LucyError::UrlNotFoundError),
        }
    }

    fn record(&mut self, record: Record) -> Result<bool, String> {
        self.store.insert(record.uuid, record.url);
        Ok(true)
    }

    fn all(&mut self) -> Vec<Record> {
        let mut records: Vec<Record> = vec![];

        for (uuid, url) in &self.store {
            match Record::from(url.to_string(), uuid.to_string()) {
                Ok(record) => records.push(record),
                Err(_) => {}
            }
        }

        records
    }
}
