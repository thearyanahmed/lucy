use crate::{datastore::Datastore, Record};
use std::collections::HashMap;
use crate::error::LucyError;

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
    fn find(&self, uuid: &str) -> Result<Record, LucyError> {
        match self.store.get(uuid) {
            Some(url) => {
                match Record::from(url.to_string(), uuid.to_string()) {
                    Ok(rec) => Ok(rec),
                    Err(_) => Err(LucyError::NotAValidError),
                }
            },
            None => Err(LucyError::UrlNotFoundError),
        }
    }

    fn record(&mut self, record: Record) -> Result<bool, String> {
        self.store.insert(record.uuid, record.url);
        Ok(true)
    }
}
