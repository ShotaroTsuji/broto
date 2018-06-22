#[macro_use]
extern crate serde_derive;
extern crate serde;

pub mod tupletype;
pub mod header;
pub mod writer;

#[cfg(test)]
mod test {
    use super::*;
    use tupletype::TupleType;

    #[test]
    fn test_primtype_parser() {
        assert_eq!(tupletype::PrimitiveType::parse("u8".as_bytes()),
            (Some(tupletype::PrimitiveType::U8), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("u16".as_bytes()),
            (Some(tupletype::PrimitiveType::U16), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("u32".as_bytes()),
            (Some(tupletype::PrimitiveType::U32), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("u64".as_bytes()),
            (Some(tupletype::PrimitiveType::U64), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("i8".as_bytes()),
            (Some(tupletype::PrimitiveType::I8), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("i16".as_bytes()),
            (Some(tupletype::PrimitiveType::I16), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("i32".as_bytes()),
            (Some(tupletype::PrimitiveType::I32), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("i64".as_bytes()),
            (Some(tupletype::PrimitiveType::I64), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("f32".as_bytes()),
            (Some(tupletype::PrimitiveType::F32), &[] as &[u8]));
        assert_eq!(tupletype::PrimitiveType::parse("f64".as_bytes()),
            (Some(tupletype::PrimitiveType::F64), &[] as &[u8]));

        assert_eq!(tupletype::PrimitiveType::parse("u8xxx".as_bytes()),
            (Some(tupletype::PrimitiveType::U8), "xxx".as_bytes()));
        assert_eq!(tupletype::PrimitiveType::parse("f32f64".as_bytes()),
            (Some(tupletype::PrimitiveType::F32), "f64".as_bytes()));

        assert_eq!(tupletype::PrimitiveType::parse("u11".as_bytes()),
            (None, "u11".as_bytes()));
    }

    fn test_ttparser_ident(s: &str) {
        let t = TupleType::parse(s.as_bytes()).0.unwrap();
        assert_eq!(t.to_string(), s);
    }

    #[test]
    fn test_tupletype() {
        test_ttparser_ident("(f32,f32)");
        test_ttparser_ident("(u64)");
        test_ttparser_ident("(u8,u16,u32,u64,[f32;8],Vec<u8>)");
        test_ttparser_ident("(i8,i16,i32,i64,f32,f64)");
        test_ttparser_ident("([i8;512],[u8;512],[f64;65536])");
    }
}
