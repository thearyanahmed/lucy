use datastore::{Datastore, DatastoreDriver};
use drivers::{hashmap_store::HashmapStore, redis_store::RedisStore};
use error::LucyError;
use record::Record;

pub mod datastore;
pub mod drivers;
pub mod error;
pub mod record;

pub struct Lucy {
    ds: Box<dyn Datastore>,
}

impl Lucy {
    pub fn new(driver: DatastoreDriver) -> Lucy {
        let ds = Lucy::get_datastore(driver);
        Lucy { ds }
    }

    pub fn find(&mut self, uuid: &str) -> Result<Record, LucyError> {
        self.ds.find(uuid)
    }

    pub fn record(&mut self, record: Record) -> Result<bool, String> {
        self.ds.record(record)
    }

    pub fn all(&mut self) -> Vec<Record> {
        self.ds.all()
    }

    fn get_datastore(driver: DatastoreDriver) -> Box<dyn Datastore> {
        match driver {
            DatastoreDriver::InMemoryHashmap => Box::new(HashmapStore::new()),
            DatastoreDriver::Redis => Box::new(RedisStore::new()),
        }
    }
}