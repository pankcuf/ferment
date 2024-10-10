use std::fmt::Debug;
use std::io;
use std::process::ExitStatus;

#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    ExpansionError(&'static str),
    MorphingError(&'static str),
    ParseSyntaxTree(syn::Error),
    Configuration(&'static str),
    Exit(ExitStatus),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileError(ref err) => std::fmt::Display::fmt(&err, f),
            Error::ExpansionError(msg) => write!(f, "{}", msg),
            Error::ParseSyntaxTree(ref err) => std::fmt::Display::fmt(&err, f),
            Error::MorphingError(ref err) => std::fmt::Display::fmt(&err, f),
            Error::Configuration(ref err) => std::fmt::Display::fmt(&err, f),
            Error::Exit(ref exit) => std::fmt::Display::fmt(&exit, f)
        }
    }
}
impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::FileError(value)
    }
}
impl From<syn::Error> for Error {
    fn from(value: syn::Error) -> Self {
        Error::ParseSyntaxTree(value)
    }
}
impl From<ExitStatus> for Error {
    fn from(value: ExitStatus) -> Self {
        Error::Exit(value)
    }
}
