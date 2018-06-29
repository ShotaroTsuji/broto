use std::io;
use std::string;
use std::fmt;
use std::result;
use std::error;

#[derive(Debug)]
pub enum Error {
    EndOfFile,
    Magic,
    UndefinedBlock,
    Io(io::Error),
    FromUtf8(string::FromUtf8Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::EndOfFile => write!(f, "End of File"),
            Error::Magic => write!(f, "Magic number error"),
            Error::UndefinedBlock => write!(f, "Undefined Block"),
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::FromUtf8(ref err) => write!(f, "String error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::EndOfFile => "End of File",
            Error::Magic => "Magic number",
            Error::UndefinedBlock => "UndefinedBlock",
            Error::Io(ref err) => err.description(),
            Error::FromUtf8(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            _ => None,
        }
    }
}
