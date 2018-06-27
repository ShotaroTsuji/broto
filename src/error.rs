use std;

#[derive(Debug)]
pub enum ReadError {
    EndOfFile,
    Magic,
    Io(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> Self {
        ReadError::Io(err)
    }
}


