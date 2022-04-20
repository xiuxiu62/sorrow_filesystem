use std::{io, time};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io,
    SystemTime,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Self::Io
    }
}

impl From<time::SystemTimeError> for Error {
    fn from(_: time::SystemTimeError) -> Self {
        Self::SystemTime
    }
}
