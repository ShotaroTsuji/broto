extern crate byteorder;

pub mod header;
pub mod writer;
pub mod reader;
pub mod error;

use std::io;
use reader::Reader;
use reader::Block;
use error::Error;
use error::Result;

pub struct Metadata {
    logs: Vec<header::LogBlock>,
}

pub fn load_float_ts<R: io::Read>(stream: R) -> Result<(Vec<(f64,Vec<f64>)>, Metadata)> {
    let mut reader = Reader::new(stream);
    let _ = reader.initialize().unwrap();

    let mut read_data = Vec::new();
    let mut metadata = Metadata { logs: Vec::new(), };

    loop {
        let result = reader.next_block();
        if let Err(e) = result {
            match e {
                Error::EndOfFile => {},
                _ => { return Err(e); },
            };
            break;
        }
        let block = result.unwrap();
        match block {
            Block::Log(log) => { metadata.logs.push(log); },
            Block::FloatTS(fts) => {
                for ent in reader.float_ts_entries(&fts) {
                    let ent = ent.unwrap();
                    read_data.push(ent);
                }
            },
        }
    }

    Ok((read_data, metadata))
}


#[cfg(test)]
mod test {
    use super::*;

}
