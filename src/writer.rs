extern crate bincode;

use std;
use std::io;
use header::{Header, BlockHeader, LogBlock};

#[derive(Debug)]
pub enum WriteError {
    Io(io::Error),
    Encode(bincode::Error),
}

impl From<io::Error> for WriteError {
    fn from(error: io::Error) -> Self {
        WriteError::Io(error)
    }
}

impl From<bincode::Error> for WriteError {
    fn from(error: bincode::Error) -> Self {
        WriteError::Encode(error)
    }
}

type Result<T> = std::result::Result<T, WriteError>;

pub struct Writer<T: io::Write> {
    stream: T,
}

impl<T: io::Write> Writer<T> {
    pub fn new(stream_: T) -> Writer<T> {
        Writer {
            stream: stream_,
        }
    }

    pub fn write_header(&mut self, file_size: u64) -> Result<usize> {
        let header = Header::new(file_size);
        let header_bin: Vec<u8> = bincode::serialize(&header)?;
        println!("fn Write::write_header");
        println!("  header        : {:?}", header);
        println!("  header binary : {:?}", header_bin);
        println!("  header size   : {}", header_bin.len());
        self.stream.write(&header_bin).map_err(|e| e.into())
    }

    pub fn write_log(&mut self, log: LogBlock) -> Result<usize> {
        let log_bin: Vec<u8> = bincode::serialize(&log)?;
        let header = BlockHeader::new("log", log_bin.len() as u64);
        let header_bin: Vec<u8> = bincode::serialize(&header)?;
        println!("fn Write::write_log");
        println!("  header        : {:?}", header);
        println!("  log           : {:?}", log);
        println!("  header binary : {:?}", header_bin);
        println!("  log binary    : {:?}", log_bin);
        println!("  header size   : {:?}", header_bin.len());
        println!("  log    size   : {:?}", log_bin.len());
        let bytes1 = self.stream.write(&header_bin)?;
        let bytes2 = self.stream.write(&log_bin)?;
        Ok(bytes1 + bytes2)
    }

    pub fn get_stream(self) -> T {
        self.stream
    }
}
