#[derive(Debug)]
pub enum Error {
    MongoError(mongodb::error::Error)
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::MongoError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;