extern crate bincode;

use std;
use std::io;
use header::Header;

#[derive(Debug)]
pub enum WriteError {
    Io(io::Error),
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
        let encoded: Vec<u8> = bincode::serialize(&header).unwrap();
        println!("header: {:?}", encoded);
        println!("header size : {}, {}", encoded.len(), std::mem::size_of::<Header>());
        let result = self.stream.write(&encoded).unwrap();
        Ok(0)
    }

    pub fn get_stream(self) -> T {
        self.stream
    }
}
