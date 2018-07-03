use std::io;
use std::marker::PhantomData;
use byteorder::{LittleEndian, ReadBytesExt};
use header::{Header, BlockHeader, LogBlock, FloatTSBlock};
use error::{Result, Error};

#[derive(Debug)]
pub enum Block {
    Log(LogBlock),
    FloatTS(FloatTSBlock),
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

    pub fn initialize(&mut self) -> Result<()> {
        let header = Header::read_from(&mut self.stream)?;
        self.header = Some(header);
        Ok(())
    }

    pub fn next_block(&mut self) -> Result<Block> {
        let bheader = BlockHeader::read_from(&mut self.stream)?;
        match bheader.clone_name().as_str() {
            "log" => LogBlock::read_from(&mut self.stream).map(|v| Block::Log(v)),
            "float-ts" => FloatTSBlock::read_from(&mut self.stream).map(|v| Block::FloatTS(v)),
            _ => Err(Error::UndefinedBlock),
        }
    }

    pub fn float_ts_entries(&mut self, data: &FloatTSBlock) -> FloatTSReader<R> {
        FloatTSReader {
            index_len: data.index_len() as usize,
            value_len: data.value_len() as usize,
            remaining: data.length().unwrap() as usize,
            stream: &mut self.stream,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct FloatTSReader<'a, R: 'a> {
    index_len : usize,
    value_len : usize,
    remaining : usize,
    stream : &'a mut R,
    phantom: PhantomData<&'a R>,
}

impl<'a, R> Iterator for FloatTSReader<'a, R> where R: 'a + io::Read {
    type Item = Result<(f64,Vec<f64>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let index: f64;
        let mut value: Vec<f64> = Vec::new();

        match self.stream.read_f64::<LittleEndian>() {
            Ok(f) => { index = f; },
            Err(e) => { return Some(Err(e.into())); },
        }
        for _ in 1..self.index_len {
            if let Err(e) = self.stream.read_f64::<LittleEndian>() {
                return Some(Err(e.into()));
            }
        }
        for _ in 0..self.value_len {
            match self.stream.read_f64::<LittleEndian>() {
                Ok(f) => { value.push(f); },
                Err(e) => { return Some(Err(e.into())); },
            }
        }
        self.remaining = self.remaining - 1;
        Some(Ok((index,value)))
    }
}
