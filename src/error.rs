use std::fmt;

#[derive(Debug)]
pub enum MockaError {
    Io(String),
    Config(String),
    Server(String),
    Http(String),
}

impl fmt::Display for MockaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MockaError::Io(err) => write!(f, "IO error: {}", err),
            MockaError::Config(err) => write!(f, "Configuration error: {}", err),
            MockaError::Server(err) => write!(f, "Server error: {}", err),
            MockaError::Http(err) => write!(f, "Http error: {}", err),
        }
    }
}

impl std::error::Error for MockaError {}
