extern crate bincode;

use std;
use std::io;
use std::marker::PhantomData;
use header::{Header, BlockHeader, LogBlock, DataBlock};
use error::ReadError;

#[derive(Debug)]
pub enum Block {
    Log(LogBlock),
    Data(DataBlock),
}

#[derive(Debug)]
pub struct Reader<R: io::Read> {
    stream: R,
    header: Option<Header>,
}

impl<R: io::Read> Reader<R> {
    pub fn new(stream: R) -> Reader<R> {
        Reader {
            stream: stream,
            header: None,
        }
    }

    pub fn initialize(&mut self) -> Result<(), ReadError> {
        let header = Header::read_from(&mut self.stream)?;
        self.header = Some(header);
        Ok(())
    }

    pub fn next_block(&mut self) -> Result<Block, ReadError> {
        let bheader = BlockHeader::read_from(&mut self.stream)?;
        //println!("block header : {:?}", bheader);
        match bheader.clone_name().as_str() {
            "log" => LogBlock::read_from(&mut self.stream).map(|v| Block::Log(v)),
            "data" => DataBlock::read_from(&mut self.stream).map(|v| Block::Data(v)),
            _ => Err(ReadError::UndefinedBlock),
        }
    }
}
