extern crate tsbin;

use std::io::Cursor;

use tsbin::header::Header;
use tsbin::header::LogBlockBuilder;
use tsbin::header::FloatTSBlockBuilder;
use tsbin::writer::Writer;
use tsbin::reader::{Reader, Block};
use tsbin::error::Error;

fn main() {
    let hd = Header::new(0);
    println!("Header: {:?}", hd);

    let log = LogBlockBuilder::new().program("tsbin").info("creation").build();
    println!("Log block: {:?}", log);

    let buf: Vec<u8> = Vec::new();
    let cur = Cursor::new(buf);
    let mut writer = Writer::new(cur);
    let _ = writer.write_header(0).unwrap();
    let _ = writer.write_log(log).unwrap();

    let data = FloatTSBlockBuilder::new()
        .index_len(1)
        .value_len(1)
        .build();
    println!("FloatTS block: {:?}", data);

    {
        let mut dw = writer.write_float_ts(data).unwrap();
        for i in 0..20 {
            let x = vec![0.1 * i as f64];
            println!("write {:?} ----> {:?}", x, dw.write_entry(i as f64, &x));
        }
    }

    let buf = writer.into_stream().into_inner();

    let cur = Cursor::new(buf);
    let mut reader = Reader::new(cur);
    let _ = reader.initialize().unwrap();

    loop {
        let result = reader.next_block();
        match result {
            Err(e) => {
                match e {
                    Error::EndOfFile => {},
                    _ => { println!("Error: {}", e); },
                }
                break;
            },
            _ => {},
        }
        let block = result.unwrap();
        match block {
            Block::Log(log) => {
                println!("Log block was found.");
                println!("    time   : {:?}", log.time());
                println!("    program: {}", log.program());
                println!("    info   : {}", log.info());
            },
            Block::FloatTS(data) => {
                println!("FloatTS block was found.");
                println!("    index_len: {}", data.index_len());
                println!("    value_len: {}", data.value_len());
                println!("    length   : {}", data.length());
                for ent in reader.float_ts_entries(&data) {
                    let ent = ent.unwrap();
                    let index = ent.0;
                    let value = ent.1;
                    print!("    {}:", index);
                    for x in value.iter() {
                        print!(" {},", x);
                    }
                    println!("");
                }
            },
        }
    }
}
