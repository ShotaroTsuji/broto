extern crate tsbin;

use std::io::Cursor;

use tsbin::header::Header;
use tsbin::header::LogBlockBuilder;
use tsbin::header::FloatTSBlockBuilder;
use tsbin::writer::Writer;
use tsbin::reader::{Reader, Block};

fn main() {
    let hd = Header::new(0);
    println!("{:?}", hd);

    let log = LogBlockBuilder::new().program("tsbin").info("creation").build();
    println!("log: {:?}", log);

    let buf: Vec<u8> = Vec::new();
    let mut writer = Writer::new(buf);
    let _ = writer.write_header(0).unwrap();
    let _ = writer.write_log(log).unwrap();

    let data = FloatTSBlockBuilder::new()
        .index_len(1)
        .value_len(1)
        .length(10)
        .build();
    println!("{:?}", data);

    {
        let mut dw = writer.write_float_ts(data).unwrap();
        println!("{:?}", dw);
        for i in 0..10 {
            let x = vec![0.1 * i as f64];
            println!("write {:?} ----> {:?}", x, dw.write_value(0.0, &x));
        }
    }

    let buf = writer.get_stream();
    println!("buf: {:?}", buf);

    let cur = Cursor::new(buf);
    let mut reader = Reader::new(cur);
    let _ = reader.initialize().unwrap();

    println!("reader: {:?}", reader);

    loop {
        let result = reader.next_block();
        match result {
            Err(e) => { break; },
            _ => {},
        }
        let block = result.unwrap();
        match block {
            Block::Log(log) => {
                println!("Log block was found.");
                println!("    program: {}", log.program());
                println!("    info   : {}", log.info());
            },
            Block::FloatTS(data) => {
                println!("FloatTS block was found.");
                println!("    index_len: {}", data.index_len());
                println!("    value_len: {}", data.value_len());
                println!("    length   : {}", data.length());
                for ent in reader.data_entries(&data) {
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
