extern crate tsbin;

use tsbin::tupletype::TupleType;
use tsbin::tupletype::PrimitiveType;
use tsbin::tupletype::Type;

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
}
