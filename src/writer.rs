use std;
use std::io;
use std::marker::PhantomData;
use byteorder::{WriteBytesExt,LittleEndian};
use header::{Header, BlockHeader, LogBlock, FloatTSBlock};
use error::WriteError;

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

    pub fn write_header(&mut self, file_size: u64) -> Result<()> {
        let header = Header::new(file_size);
        header.write_into(&mut self.stream)
    }

    pub fn write_log(&mut self, log: LogBlock) -> Result<()> {
        let header = BlockHeader::new("log", log.size() as u64);
        header.write_into(&mut self.stream)?;
        log.write_into(&mut self.stream)?;
        Ok(())
    }

    pub fn write_float_ts(&mut self, block: FloatTSBlock) -> Result<FloatTSWriter<W>> {
        let header = BlockHeader::new("float-ts", block.size() as u64);
        header.write_into(&mut self.stream)?;
        block.write_into(&mut self.stream)?;
        Ok(FloatTSWriter {
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
pub struct FloatTSWriter<'a, W: 'a> {
    value_len : usize,
    stream : &'a mut W,
    phantom: PhantomData<&'a W>,
}

impl<'a, W> FloatTSWriter<'a, W> where W: 'a + std::io::Write {
    pub fn write_value(&mut self, index: f64, values: &[f64]) -> Result<()> {
        let _ = self.stream.write_f64::<LittleEndian>(index)?;
        for &x in values.iter() {
            let _ = self.stream.write_f64::<LittleEndian>(x)?;
        }
        Ok(())
    }
}
