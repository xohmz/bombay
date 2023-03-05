use std::fmt::Display;

/// Bombay error type.
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Request(ureq::Error),
    Deserialization(serde_json::Error),
    Message(&'static str),
    NotFound(&'static str),
    SignIn(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(io_err) => write!(f, "{}", io_err),
            Error::Request(req_err) => write!(f, "{}", req_err),
            Error::Deserialization(serde_err) => write!(f, "{}", serde_err),
            Error::Message(str_err) => write!(f, "{}", str_err),
            Error::NotFound(item) => write!(f, "Could not find {}.", item),
            Error::SignIn(str_err) => write!(f, "Could not sign in. {}.", str_err),
        }
    }
}

impl std::error::Error for Error {}
