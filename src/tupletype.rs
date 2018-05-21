use std::string;

pub struct TupleTypeBuilder {
    types: Vec<Type>,
}

impl TupleTypeBuilder {
    pub fn new() -> Self {
        TupleTypeBuilder { types: Vec::new() }
    }

    pub fn push(mut self, t: Type) -> TupleTypeBuilder {
        self.types.push(t);
        self
    }

    pub fn push_str(mut self, t: &str) -> TupleTypeBuilder {
        let (result, _) = Type::parse(t.as_bytes());
        self.types.push(result.unwrap());
        self
    }

    pub fn build(self) -> TupleType {
        assert!(self.types.len() != 0);
        TupleType { types: self.types }
    }
}

#[derive(Debug,PartialEq)]
pub struct TupleType {
    types: Vec<Type>,
}

impl string::ToString for TupleType {
    fn to_string(&self) -> String {
        let mut s = String::from("(");
        s.push_str(&self.types[0].to_string());
        for t in self.types[1..].iter() {
            s.push(',');
            s.push_str(&t.to_string());
        }
        s.push(')');
        s
    }
}

impl TupleType {
    pub fn parse(data: &[u8]) -> (Option<Self>, &[u8]) {
        let mut types_ = Vec::new();

        // read a character '('
        let head = data;
        if head.len() == 0 {
            return (None, data);
        }
        let c = head[0] as char;
        if c != '(' {
            return (None, data);
        }

        let mut head = &head[1..];
        while head.len() > 0 {
            // read a Type
            let (result, head_new) = Type::parse(head);
            if result.is_none() {
                return (None, data);
            }
            types_.push(result.unwrap());
            head = head_new;

            // read a delimiter or trailer
            if head.len() == 0 {
                return (None, data);
            }
            let c = head[0] as char;
            match c {
                ')' => { head = &head[1..]; break; },
                ',' => { head = &head[1..]; }
                _   => { return (None, data); }
            }
        }

        (Some(TupleType { types: types_ }), head)
    }
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug,PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Array(PrimitiveType, u64),
    Vector(PrimitiveType),
}

impl string::ToString for PrimitiveType {
    fn to_string(&self) -> String {
        match self {
            PrimitiveType::U8  => String::from("u8"),
            PrimitiveType::U16 => String::from("u16"),
            PrimitiveType::U32 => String::from("u32"),
            PrimitiveType::U64 => String::from("u64"),
            PrimitiveType::I8  => String::from("i8"),
            PrimitiveType::I16 => String::from("i16"),
            PrimitiveType::I32 => String::from("i32"),
            PrimitiveType::I64 => String::from("i64"),
            PrimitiveType::F32 => String::from("f32"),
            PrimitiveType::F64 => String::from("f64"),
        }
    }
}

impl string::ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Primitive(pt) => pt.to_string(),
            Type::Array(pt, len) => format!("[{};{}]", pt.to_string(), len),
            Type::Vector(pt) => format!("Vec<{}>", pt.to_string()),
        }
    }
}

impl PrimitiveType {
    pub fn parse(data: &[u8]) -> (Option<Self>, &[u8]) {
        let (result, head) = Self::parse_prefix(data);
        if result.is_none() {
            return (None, data);
        }
        let prefix = result.unwrap();

        let (result, head) = Self::parse_size(head);
        if result.is_none() {
            return (None, data);
        }
        let size = result.unwrap();

        match (prefix, size) {
            ('u', 8)  => (Some(PrimitiveType::U8),  head),
            ('u', 16) => (Some(PrimitiveType::U16), head),
            ('u', 32) => (Some(PrimitiveType::U32), head),
            ('u', 64) => (Some(PrimitiveType::U64), head),
            ('i', 8)  => (Some(PrimitiveType::I8),  head),
            ('i', 16) => (Some(PrimitiveType::I16), head),
            ('i', 32) => (Some(PrimitiveType::I32), head),
            ('i', 64) => (Some(PrimitiveType::I64), head),
            ('f', 32) => (Some(PrimitiveType::F32), head),
            ('f', 64) => (Some(PrimitiveType::F64), head),
            _ => (None, data),
        }
    }

    pub fn parse_prefix(data: &[u8]) -> (Option<char>, &[u8]) {
        match data[0] as char {
            'u' => (Some('u'), &data[1..]),
            'i' => (Some('i'), &data[1..]),
            'f' => (Some('f'), &data[1..]),
            _ => (None, &data),
        }
    }

    pub fn parse_size(data: &[u8]) -> (Option<u64>, &[u8]) {
        let mut head = data;
        let mut size = 0;
        while head.len() > 0 {
            let c = head[0] as char;
            if let Some(n) = c.to_digit(10) {
                let n = n as u64;
                size = size * 10 + n;
                head = &head[1..];
            } else {
                break;
            }
        }
        if head == data {
            (None, data)
        } else {
            (Some(size), head)
        }
    }
}

impl Type {
    pub fn parse(data: &[u8]) -> (Option<Self>, &[u8]) {
        let (result, head) = PrimitiveType::parse(data);
        if let Some(pt) = result {
            return (Some(Type::Primitive(pt)), head);
        }

        let (result, head) = Type::parse_array(data);
        if result.is_some() {
            return (result, head);
        }

        let (result, head) = Type::parse_vector(data);
        if result.is_some() {
            return (result, head);
        }

        (None, data)
    }

    pub fn parse_array(data: &[u8]) -> (Option<Self>, &[u8]) {
        let head = data;

        // read a character '['
        if head.len() == 0 {
            return (None, data);
        }
        let c = head[0] as char;
        if c != '[' {
            return (None, data);
        }

        // read a PrimitiveType
        let head = &head[1..];
        let (result, head) = PrimitiveType::parse(head);
        if result.is_none() {
            return (None, data);
        }
        let pt = result.unwrap();

        // read a character ';'
        if head.len() == 0 {
            return (None, data);
        }
        let c = head[0] as char;
        if c != ';' {
            return (None, data);
        }

        // read the size
        let head = &head[1..];
        let (result, head) = PrimitiveType::parse_size(head);
        if result.is_none() {
            return (None, data);
        }
        let size = result.unwrap();

        // read a character ']'
        if head.len() == 0 {
            return (None, data);
        }
        let c = head[0] as char;
        if c != ']' {
            return (None, data);
        }
        let head = &head[1..];

        (Some(Type::Array(pt, size)), head)
    }

    pub fn parse_vector(data: &[u8]) -> (Option<Self>, &[u8]) {
        let head = data;

        // read a string "Vec<"
        if head.len() < 4 {
            return (None, data);
        }
        if head[0..4].iter().ne("Vec<".as_bytes().iter()) {
            return (None, data);
        }

        // read a PrimitiveType
        let head = &head[4..];
        let (result, head) = PrimitiveType::parse(head);
        if result.is_none() {
            return (None, data);
        }
        let pt = result.unwrap();

        // read a character '>'
        if head.len() == 0 {
            return (None, data);
        }
        let c = head[0] as char;
        if c != '>' {
            return (None, data);
        }
        let head = &head[1..];

        (Some(Type::Vector(pt)), head)
    }
}
