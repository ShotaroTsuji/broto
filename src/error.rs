use std;

#[derive(Debug)]
pub enum Error {
    EndOfFile,
    Magic,
    UndefinedBlock,
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
