extern crate tsbin;

use std::io::Cursor;

use tsbin::tupletype::TupleTypeBuilder;
use tsbin::tupletype::TupleType;
use tsbin::tupletype::PrimitiveType;
use tsbin::tupletype::Type;
use tsbin::header::Header;
use tsbin::header::BlockHeader;
use tsbin::header::LogBlock;
use tsbin::header::LogBlockBuilder;
use tsbin::header::{DataBlock,DataBlockBuilder};
use tsbin::writer::Writer;
use tsbin::reader::Reader;

fn main() {
    let pt1 = PrimitiveType::U8;
    let pt2 = PrimitiveType::F64;
    println!("{}", pt1.to_string());
    println!("{}", pt2.to_string());

    let t1 = Type::Primitive(PrimitiveType::F64);
    let t2 = Type::Array(PrimitiveType::F32, 4);
    let t3 = Type::Vector(PrimitiveType::U32);
    println!("{}", t1.to_string());
    println!("{}", t2.to_string());
    println!("{}", t3.to_string());

    let str1 = "u8";
    let str2 = "x16";
    println!("{:?}", PrimitiveType::parse_prefix(str1.as_bytes()));
    println!("{:?}", PrimitiveType::parse_prefix(str2.as_bytes()));

    let str3 = "8";
    let str4 = "32";
    let str5 = "152";
    let str6 = "6a0";
    println!("{:?}", PrimitiveType::parse_size(str3.as_bytes()));
    println!("{:?}", PrimitiveType::parse_size(str4.as_bytes()));
    println!("{:?}", PrimitiveType::parse_size(str5.as_bytes()));
    println!("{:?}", PrimitiveType::parse_size(str6.as_bytes()));

    let str7 = "u8";
    let str8 = "f32";
    println!("{:?}", PrimitiveType::parse(str7.as_bytes()));
    println!("{:?}", PrimitiveType::parse(str8.as_bytes()));

    let str9 = "[f32;4]";
    let str10 = "[f64;128]";
    let str11 = "[u32:8]";
    let str12 = "[str;8]";
    println!("{:?}", Type::parse_array(str9.as_bytes()));
    println!("{:?}", Type::parse_array(str10.as_bytes()));
    println!("{:?}", Type::parse_array(str11.as_bytes()));
    println!("{:?}", Type::parse_array(str12.as_bytes()));

    let str13 = "Vec<u8>";
    let str14 = "Vec<f32>";
    println!("{:?}", Type::parse_vector(str13.as_bytes()));
    println!("{:?}", Type::parse_vector(str14.as_bytes()));

    let tt = TupleTypeBuilder::new()
                 .push(Type::Primitive(PrimitiveType::F32)).build();
    println!("{}", tt.to_string());

    let tt = TupleTypeBuilder::new()
                 .push(Type::Primitive(PrimitiveType::F32))
                 .push(Type::Primitive(PrimitiveType::U8))
                 .push(Type::Primitive(PrimitiveType::U32)).build();
    println!("{}", tt.to_string());

    let tt = TupleTypeBuilder::new()
                 .push(Type::Primitive(PrimitiveType::F32))
                 .push(Type::Array(PrimitiveType::U32, 16)).build();
    println!("{}", tt.to_string());

    let tt = TupleTypeBuilder::new()
                 .push_str("f32")
                 .push_str("[u32;16]").build();
    println!("{}", tt.to_string());

    let tt = TupleTypeBuilder::new()
                 .push_str("f64")
                 .push_str("Vec<u32>").build();
    println!("{}", tt.to_string());

    println!("{:?}", TupleType::parse("(u8)".as_bytes()));
    println!("{:?}", TupleType::parse("(u8,u64,i32)".as_bytes()));
    println!("{:?}", TupleType::parse("(f32,[f32;4])".as_bytes()));

    let hd = Header::new(0);
    println!("{:?}", hd);

    let log = LogBlockBuilder::new().program("tsbin").info("creation").build();
    println!("log: {:?}", log);

    let buf: Vec<u8> = Vec::new();
    let mut writer = Writer::new(buf);
    let size = writer.write_header(0).unwrap();
    println!("written size = {}", size);
    let size = writer.write_log(log).unwrap();
    println!("written size = {}", size);

    let data = DataBlockBuilder::new()
        .index_len(1)
        .value_len(1)
        .length(10)
        .build();
    println!("{:?}", data);

    {
        let mut dw = writer.write_data(data).unwrap();
        println!("{:?}", dw);
        for i in 0..10 {
            let x = vec![0.1 * i as f64];
            println!("write {:?} ----> {:?}", x, dw.write_value(0.0, &x));
        }
    }

    let buf = writer.get_stream();
    println!("buf: {:?}", buf);

    let mut cur = Cursor::new(buf);
    let mut reader = Reader::new(cur);
    let _ = reader.initialize().unwrap();

    println!("reader: {:?}", reader);

    let block = reader.next_block().unwrap();
    println!("block : {:?}", block);

    /*
    let hd = Header::read_from(&mut cur).unwrap();
    println!("header : {:?}", hd);

    let hd = BlockHeader::read_from(&mut cur).unwrap();
    println!("block header : {:?}", hd);

    let log = LogBlock::read_from(&mut cur).unwrap();
    println!("log : {:?}", log);

    let hd = BlockHeader::read_from(&mut cur).unwrap();
    println!("block header : {:?}", hd);

    let data = DataBlock::read_from(&mut cur).unwrap();
    println!("data block : {:?}", data);
    */
}
