use crate::record::Record;
use crate::error::LucyError;

pub enum DatastoreDriver {
    InMemoryHashmap,
    Redis,
    // MySQL,
    // PostgreSQL,
    // Redis,
}

pub trait Datastore {
    fn find(&mut self, uuid: &str) -> Result<Record, LucyError>;

    fn record(&mut self, record: Record) -> Result<bool, String>;

    fn all(&mut self) -> Vec<Record>;
}
