use std::io;
use std::io::SeekFrom;
use byteorder::{WriteBytesExt,LittleEndian};
use header::{Header, BlockHeader, LogBlock, F64TSBlock};
use error::Result;


#[derive(Debug)]
pub struct Writer<W: io::Write> {
    stream: W,
}

impl<W: io::Write> Writer<W> {
    pub fn new(stream: W) -> Writer<W> {
        Writer {
            stream: stream,
        }
    }

    pub fn write_header(&mut self) -> Result<()> {
        let header = Header::new();
        header.write_into(&mut self.stream)
    }

    pub fn write_log(&mut self, log: &LogBlock) -> Result<()> {
        let header = BlockHeader::new("log", log.size() as u64);
        header.write_into(&mut self.stream)?;
        log.write_into(&mut self.stream)?;
        Ok(())
    }

    pub fn stream_mut(&mut self) -> &mut W {
        &mut self.stream
    }

    pub fn into_stream(self) -> W {
        self.stream
    }

    pub fn write_f64ts(mut self, block: F64TSBlock) -> Result<F64TSWriter<W>> {
        assert!(block.length().is_some());
        let header = BlockHeader::new("f64ts", block.size() as u64);
        header.write_into(&mut self.stream)?;
        block.write_into(&mut self.stream)?;
        Ok(F64TSWriter {
            value_len: block.value_len() as usize,
            writer : self,
            block_header: block,
            block_pos: None,
            count: 0,
            finalized: true,
        })
    }
}

impl<W: io::Write + io::Seek> Writer<W> {
    pub fn write_f64ts_with_seek(mut self, block: F64TSBlock) -> Result<F64TSWriter<W>> {
        let header = BlockHeader::new("f64ts", block.size() as u64);
        header.write_into(&mut self.stream)?;
        let block_pos = self.stream.seek(SeekFrom::Current(0))?;
        block.write_into(&mut self.stream)?;
        Ok(F64TSWriter {
            value_len: block.value_len() as usize,
            writer: self,
            block_header: block,
            block_pos: Some(block_pos),
            count: 0,
            finalized: false,
        })
    }
}

#[derive(Debug)]
pub struct F64TSWriter<W> where W: io::Write {
    value_len : usize,
    writer : Writer<W>,
    block_header : F64TSBlock,
    block_pos : Option<u64>,
    count : u64,
    finalized : bool,
}

impl<W> F64TSWriter<W> where W: io::Write {
    pub fn stream_mut(&mut self) -> &mut W {
        self.writer.stream_mut()
    }

    pub fn write_entry(&mut self, index: f64, values: &[f64]) -> Result<()> {
        let _ = self.stream_mut().write_f64::<LittleEndian>(index)?;
        for &x in values.iter() {
            let _ = self.stream_mut().write_f64::<LittleEndian>(x)?;
        }
        self.count = self.count + 1;
        Ok(())
    }

    pub fn finish(self) -> Writer<W> {
        assert!(self.finalized, "fn finalize() must be called");
        self.writer
    }
}

impl<W> F64TSWriter<W> where W: io::Write + io::Seek {
    pub fn finalize(mut self) -> Result<Self> {
        let block_pos = self.block_pos.unwrap();
        let mut block_header = self.block_header.clone();
        let count = self.count;
        {
            let stream = self.stream_mut();
            let current = stream.seek(SeekFrom::Current(0))?;
            stream.seek(SeekFrom::Start(block_pos))?;
            block_header.set_length(count);
            block_header.write_into(stream)?;
            stream.seek(SeekFrom::Start(current))?;
        }
        self.block_header = block_header;
        self.finalized = true;
        Ok(self)
    }
}
