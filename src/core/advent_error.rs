use std::io;

pub enum AdventError {
    IoError(io::Error),
    OtherError(String),
}

impl From<io::Error> for AdventError {
    fn from(e: io::Error) -> Self {
        AdventError::IoError(e)
    }
}
