use std::io;
use std::io::SeekFrom;
use std::marker::PhantomData;
use byteorder::{WriteBytesExt,LittleEndian};
use header::{Header, BlockHeader, LogBlock, FloatTSBlock};
use error::Result;


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

    pub fn write_log(&mut self, log: &LogBlock) -> Result<()> {
        let header = BlockHeader::new("log", log.size() as u64);
        header.write_into(&mut self.stream)?;
        log.write_into(&mut self.stream)?;
        Ok(())
    }

    pub fn into_stream(self) -> W {
        self.stream
    }

    pub fn write_float_ts(&mut self, block: FloatTSBlock) -> Result<FloatTSWriter<W>> {
        assert!(block.length().is_some());
        let header = BlockHeader::new("float-ts", block.size() as u64);
        header.write_into(&mut self.stream)?;
        block.write_into(&mut self.stream)?;
        Ok(FloatTSWriter {
            value_len: block.value_len() as usize,
            stream: &mut self.stream,
            block_header: block,
            block_pos: None,
            count: 0,
            phantom: PhantomData,
            finalized: true,
        })
    }
}

impl<W: io::Write + io::Seek> Writer<W> {
    pub fn write_float_ts_with_seek(&mut self, block: FloatTSBlock) -> Result<FloatTSWriter<W>> {
        let header = BlockHeader::new("float-ts", block.size() as u64);
        header.write_into(&mut self.stream)?;
        let block_pos = self.stream.seek(SeekFrom::Current(0))?;
        block.write_into(&mut self.stream)?;
        Ok(FloatTSWriter {
            value_len: block.value_len() as usize,
            stream: &mut self.stream,
            block_header: block,
            block_pos: Some(block_pos),
            count: 0,
            phantom: PhantomData,
            finalized: false,
        })
    }
}

#[derive(Debug)]
pub struct FloatTSWriter<'a, W: 'a> where W: 'a + io::Write {
    value_len : usize,
    stream : &'a mut W,
    block_header : FloatTSBlock,
    block_pos : Option<u64>,
    count : u64,
    phantom: PhantomData<&'a W>,
    finalized : bool,
}

impl<'a, W> FloatTSWriter<'a, W> where W: 'a + io::Write {
    pub fn write_entry(&mut self, index: f64, values: &[f64]) -> Result<()> {
        let _ = self.stream.write_f64::<LittleEndian>(index)?;
        for &x in values.iter() {
            let _ = self.stream.write_f64::<LittleEndian>(x)?;
        }
        self.count = self.count + 1;
        Ok(())
    }
}

impl<'a, W> FloatTSWriter<'a, W> where W: 'a + io::Write + io::Seek {
    pub fn finalize(&mut self) -> Result<()> {
        let current = self.stream.seek(SeekFrom::Current(0))?;
        self.stream.seek(SeekFrom::Start(self.block_pos.unwrap()))?;
        self.block_header.set_length(self.count);
        self.block_header.write_into(self.stream)?;
        self.stream.seek(SeekFrom::Start(current))?;
        self.finalized = true;
        Ok(())
    }
}

impl<'a, W> Drop for FloatTSWriter<'a, W> where W: 'a + io::Write {
    fn drop(&mut self) {
        assert!(self.finalized, "fn finalize() must be called");
    }
}
