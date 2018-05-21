pub mod tupletype;

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
