use crate::Record;
use crate::error::LucyError;

pub trait Datastore {
    fn find(&self, uuid: &str) -> Result<Record, LucyError>;

    fn record(&mut self, record: Record) -> Result<bool, String>;
}
