use std::io;

#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    ExpansionError(&'static str)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileError(ref err) => err.fmt(f),
            Error::ExpansionError(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::FileError(value)
    }
}

impl std::error::Error for Error {}

