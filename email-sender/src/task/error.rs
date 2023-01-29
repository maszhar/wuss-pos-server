use mongodb::{bson::oid, error};

#[derive(Debug)]
pub enum Error {
    IdError(oid::Error),
    MongoError(error::Error)
}

impl From<oid::Error> for Error {
    fn from(value: oid::Error) -> Self {
        Self::IdError(value)
    }
}

impl From<error::Error> for Error {
    fn from(value: error::Error) -> Self {
        Self::MongoError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;