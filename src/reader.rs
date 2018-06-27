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

    pub fn initialize(&mut self) -> Option<()> {
        let header = Header::read_from(&mut self.stream).unwrap();
        self.header = Some(header);
        Some(())
    }

    pub fn next_block(&mut self) -> Option<Block> {
        let bheader = BlockHeader::read_from(&mut self.stream).unwrap();
        println!("block header : {:?}", bheader);
        match bheader.clone_name().as_str() {
            "log" => Some(Block::Log(LogBlock::read_from(&mut self.stream).unwrap())),
            "data" => Some(Block::Data(DataBlock::read_from(&mut self.stream).unwrap())),
            _ => None,
        }
    }
}
