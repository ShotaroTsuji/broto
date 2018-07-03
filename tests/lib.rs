extern crate tsbin;

use std::io::Cursor;

use tsbin::header::Header;
use tsbin::header::LogBlockBuilder;
use tsbin::header::FloatTSBlockBuilder;
use tsbin::writer::Writer;
use tsbin::reader::{Reader, Block};
use tsbin::error::Error;

#[test]
fn test_reader_writer_1() {
    let mut data = Vec::new();

    for i in 0..30 {
        let x = i as f64;
        data.push(vec![0.1 * x, 0.2 * x, 0.3 * x]);
    }

    let hd = Header::new(0);
    println!("Header: {:?}", hd);

    let log = LogBlockBuilder::new().program("tsbin").info("creation").build();
    println!("Log block: {:?}", log);

    let buf: Vec<u8> = Vec::new();
    let cur = Cursor::new(buf);
    let mut writer = Writer::new(cur);
    let _ = writer.write_header(0).unwrap();
    let _ = writer.write_log(&log).unwrap();

    let fts = FloatTSBlockBuilder::new()
        .index_len(1)
        .value_len(3)
        .build();
    println!("FloatTS block: {:?}", fts);

    {
        let mut w = writer.write_float_ts_with_seek(fts).unwrap();
        for (i, v) in data.iter().enumerate() {
            println!("write {:?} ----> {:?}", *v, w.write_entry(i as f64, v));
        }
        w.finalize().unwrap();
    }

    let buf = writer.into_stream().into_inner();

    let cur = Cursor::new(buf);
    let mut reader = Reader::new(cur);
    let _ = reader.initialize().unwrap();

    let mut read_data = Vec::new();

    loop {
        let result = reader.next_block();
        if let Err(e) = result {
            match e {
                Error::EndOfFile => {},
                _ => { println!("Error: {}", e); },
            };
            break;
        }
        let block = result.unwrap();
        match block {
            Block::Log(log) => {
                println!("Log block was found.");
                println!("    time   : {:?}", log.time());
                println!("    program: {}", log.program());
                println!("    info   : {}", log.info());
            },
            Block::FloatTS(fts) => {
                println!("FloatTS block was found.");
                println!("    index_len: {}", fts.index_len());
                println!("    value_len: {}", fts.value_len());
                println!("    length   : {}", fts.length().unwrap());
                for ent in reader.float_ts_entries(&fts) {
                    let ent = ent.unwrap();
                    let index = ent.0;
                    let value = ent.1;
                    print!("    {}:", index);
                    for x in value.iter() {
                        print!(" {},", x);
                    }
                    println!("");
                    read_data.push(value);
                }
            },
        }
    }

    assert_eq!(data, read_data);
}

#[test]
fn test_reader_writer_2() {
    let mut data = Vec::new();

    for i in 0..30 {
        let x = i as f64;
        data.push(vec![0.1 * x, 0.2 * x, 0.3 * x]);
    }

    let hd = Header::new(0);
    println!("Header: {:?}", hd);

    let log = LogBlockBuilder::new().program("tsbin").info("creation").build();
    println!("Log block: {:?}", log);

    let buf: Vec<u8> = Vec::new();
    let cur = Cursor::new(buf);
    let mut writer = Writer::new(cur);
    let _ = writer.write_header(0).unwrap();
    let _ = writer.write_log(&log).unwrap();

    let fts = FloatTSBlockBuilder::new()
        .index_len(1)
        .value_len(3)
        .build();
    println!("FloatTS block: {:?}", fts);

    {
        let mut w = writer.write_float_ts_with_seek(fts).unwrap();
        for (i, v) in data.iter().enumerate() {
            println!("write {:?} ----> {:?}", *v, w.write_entry(i as f64, v));
        }
        w.finalize().unwrap();
    }

    let buf = writer.into_stream().into_inner();

    let cur = Cursor::new(buf);
    let (entries, _) = tsbin::load_float_ts(cur).unwrap();
    let read_data: Vec<Vec<f64>> = entries.into_iter().map(|(_, v)| v).collect();

    assert_eq!(data, read_data);
}

#[test]
fn test_reader_writer_3() {
    let buf: Vec<u8> = Vec::new();
    let mut data = Vec::new();

    for i in 0..1000 {
        let x = i as f64;
        data.push((x, vec![0.1 * x, 0.2 * x, 0.3 * x]));
    }

    let mut metadata = tsbin::Metadata::new();
    let log = LogBlockBuilder::new().program("tsbin").info("creation").build();
    metadata.get_logs_mut().push(log);
    let log = LogBlockBuilder::new().program("tsbin").info("comment").build();
    metadata.get_logs_mut().push(log);

    let cur = Cursor::new(buf);

    let mut cur = tsbin::save_float_ts(cur, &data, &metadata).unwrap();
    cur.set_position(0);

    let (entries, read_meta) = tsbin::load_float_ts(cur).unwrap();

    assert_eq!(data, entries);
    assert_eq!(metadata, read_meta);
}
