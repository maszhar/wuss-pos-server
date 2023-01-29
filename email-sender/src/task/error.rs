use super::model;

#[derive(Debug)]
pub enum Error {
    NotFound(),
    TaskModelError(model::error::Error)
}

impl From<model::error::Error> for Error {
    fn from(value: model::error::Error) -> Self {
        Self::TaskModelError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;