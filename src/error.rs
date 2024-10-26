use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ShellError {
    Io(io::Error),
    CommandNotFound(String),
    EnvVarError(String),
    ParseError(String),
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellError::Io(e) => write!(f, "I/O error: {}", e),
            ShellError::CommandNotFound(cmd) => write!(f, "Command not found: {}", cmd),
            ShellError::EnvVarError(var) => write!(f, "Environment variable error: {}", var),
            ShellError::ParseError(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for ShellError {}

impl From<io::Error> for ShellError {
    fn from(error: io::Error) -> Self {
        ShellError::Io(error)
    }
}
