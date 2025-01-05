use std::{fmt, io};

#[derive(Debug)]
pub enum MockaError {
    Io(io::Error),
    Config(String),
    Server(String),
}

impl fmt::Display for MockaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MockaError::Io(err) => write!(f, "IO error: {}", err),
            MockaError::Config(err) => write!(f, "Configuration error: {}", err),
            MockaError::Server(err) => write!(f, "Server error: {}", err),
        }
    }
}

impl std::error::Error for MockaError {}

impl From<io::Error> for MockaError {
    fn from(err: io::Error) -> Self {
        MockaError::Io(err)
    }
}
