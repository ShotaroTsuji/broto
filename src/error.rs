use std;

#[derive(Debug)]
pub enum ReadError {
    EndOfFile,
    Magic,
    UndefinedBlock,
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self {
        ReadError::Io(err)
    }
}

#[derive(Debug)]
pub enum WriteError {
    EndOfFile,
    Magic,
    Io(std::io::Error),
}

impl From<std::io::Error> for WriteError {
    fn from(error: std::io::Error) -> Self {
        WriteError::Io(error)
    }
}
