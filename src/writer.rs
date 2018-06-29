extern crate bincode;

use std;
use std::io;
use std::marker::PhantomData;
use byteorder::{WriteBytesExt,LittleEndian};
use header::{Header, BlockHeader, LogBlock, DataBlock};
use serde;

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

pub struct Writer<W: io::Write> {
    stream: W,
}

impl<W: io::Write> Writer<W> {
    pub fn new(stream: W) -> Writer<W> {
        Writer {
            stream: stream,
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

    pub fn write_data(&mut self, block: DataBlock) -> Result<DataWriter<W>> {
        let block_bin: Vec<u8> = bincode::serialize(&block)?;
        let header = BlockHeader::new("data", block_bin.len() as u64);
        let header_bin: Vec<u8> = bincode::serialize(&header)?;
        self.stream.write(&header_bin)?;
        self.stream.write(&block_bin)?;
        Ok(DataWriter {
            value_len: block.value_len() as usize,
            stream: &mut self.stream,
            phantom: PhantomData,
        })
    }

    pub fn get_stream(self) -> W {
        self.stream
    }
}

#[derive(Debug)]
pub struct DataWriter<'a, W: 'a> {
    value_len : usize,
    stream : &'a mut W,
    phantom: PhantomData<&'a W>,
}

impl<'a, W> DataWriter<'a, W> where W: 'a + std::io::Write {
    pub fn write_value(&mut self, index: f64, values: &[f64]) -> Result<()> {
        let _ = self.stream.write_f64::<LittleEndian>(index)?;
        for &x in values.iter() {
            let _ = self.stream.write_f64::<LittleEndian>(x)?;
        }
        Ok(())
    }
}
