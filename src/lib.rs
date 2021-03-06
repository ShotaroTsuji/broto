extern crate byteorder;

pub mod header;
pub mod writer;
pub mod reader;
pub mod error;

pub use self::writer::*;
pub use self::reader::*;
pub use self::header::*;
pub use self::error::*;

use std::io;

#[derive(Debug,PartialEq)]
pub struct Metadata {
    logs: Vec<header::LogBlock>,
}

impl Metadata {
    pub fn new() -> Self {
        Metadata {
            logs: Vec::new(),
        }
    }

    pub fn get_logs(&self) -> &Vec<header::LogBlock> {
        &self.logs
    }

    pub fn get_logs_mut(&mut self) -> &mut Vec<header::LogBlock> {
        &mut self.logs
    }
}

pub fn load_f64ts<R: io::Read>(stream: R) -> Result<(Vec<(f64,Vec<f64>)>, Metadata)> {
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
            Block::F64TS(fts) => {
                for ent in reader.f64ts_entries(&fts) {
                    let ent = ent.unwrap();
                    read_data.push(ent);
                }
            },
        }
    }

    Ok((read_data, metadata))
}

pub fn save_f64ts<W: io::Write>(stream: W, entries: &[(f64,Vec<f64>)], metadata: &Metadata) -> Result<W> {
    let mut writer = Writer::new(stream);
    writer.write_header()?;

    for log in metadata.logs.iter() {
        writer.write_log(log)?;
    }

    let fts = F64TSBlockBuilder::new()
        .index_len(1)
        .value_len(entries[0].1.len() as u64)
        .length(entries.len() as u64)
        .build();
    println!("F64TS block: {:?}", fts);

    let mut w = writer.write_f64ts(fts).unwrap();
    for entry in entries.iter() {
        let index = entry.0;
        let value = &entry.1;
        w.write_entry(index, value)?;
    }
    let writer = w.finish();

    Ok(writer.into_stream())
}


