use std::io;

#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    ExpansionError(&'static str),
    MorphingError(&'static str),
    ParseSyntaxTree(syn::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileError(ref err) => err.fmt(f),
            Error::ExpansionError(msg) => write!(f, "{}", msg),
            Error::ParseSyntaxTree(ref err) => err.fmt(f),
            Error::MorphingError(ref err) => err.fmt(f),
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
