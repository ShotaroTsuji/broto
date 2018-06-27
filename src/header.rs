use tupletype::TupleType;
use std;
use byteorder::{LittleEndian, ReadBytesExt};

fn read_string_from<R: std::io::Read>(reader: &mut R) -> Option<String> {
    let mut v = Vec::new();
    let len = reader.read_u64::<LittleEndian>().unwrap();
    for _ in 0..len {
        let c = reader.read_u8().unwrap();
        v.push(c);
    }
    Some(String::from_utf8(v).unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    magic_number    : [u8; 8],
    header_size     : u64,
    major_version   : u32,
    minor_version   : u32,
    file_size       : u64,
}

impl Header {
    pub fn new(file_size: u64) -> Header {
        let mut magic: [u8; 8] = [0; 8];
        magic.clone_from_slice("tsbinfmt".as_bytes());
        Header {
            magic_number  : magic,
            header_size   : std::mem::size_of::<Header>() as u64,
            major_version : 0,
            minor_version : 1,
            file_size     : file_size,
        }
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Option<Header> {
        let mut magic: [u8; 8] = [0; 8];
        let result = reader.read(&mut magic);
        assert_eq!(result.unwrap(), 8);
        let header_size = reader.read_u64::<LittleEndian>().unwrap();
        let major_version = reader.read_u32::<LittleEndian>().unwrap();
        let minor_version = reader.read_u32::<LittleEndian>().unwrap();
        let file_size = reader.read_u64::<LittleEndian>().unwrap();
        let hd = Header {
            magic_number : magic,
            header_size  : header_size,
            major_version: major_version,
            minor_version: minor_version,
            file_size    : file_size,
        };
        Some(hd)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    magic : [u8; 8],
    name  : String,
    size  : u64,
}

impl BlockHeader {
    pub fn new<S: Into<String>>(name: S, size: u64) -> Self {
        let mut magic: [u8; 8] = [0; 8];
        magic.clone_from_slice("block   ".as_bytes());
        BlockHeader {
            magic : magic,
            name  : name.into(),
            size  : size,
        }
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Option<Self> {
        let mut magic: [u8; 8] = [0; 8];
        let result = reader.read(&mut magic);
        assert_eq!(result.unwrap(), 8);
        let name = read_string_from(reader).unwrap();
        let size = reader.read_u64::<LittleEndian>().unwrap();
        let hd = BlockHeader {
            magic : magic,
            name  : name,
            size  : size,
        };
        Some(hd)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct DataBlock {
    index_len  : u64,
    value_len  : u64,
    length     : u64,
}

impl DataBlock {
    pub fn index_len(&self) -> u64 {
        self.index_len
    }

    pub fn value_len(&self) -> u64 {
        self.value_len
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Option<Self> {
        let index_len = reader.read_u64::<LittleEndian>().unwrap();
        let value_len = reader.read_u64::<LittleEndian>().unwrap();
        let length = reader.read_u64::<LittleEndian>().unwrap();
        Some(DataBlock {
            index_len: index_len,
            value_len: value_len,
            length: length,
        })
    }
}

#[derive(Debug)]
pub struct DataBlockBuilder<IdxLenType,ValLenType,LengthType> {
    index_len : IdxLenType,
    value_len : ValLenType,
    length    : LengthType,
}

impl DataBlockBuilder<(), (), ()> {
    pub fn new() -> Self {
        DataBlockBuilder {
            index_len : (),
            value_len : (),
            length : (),
        }
    }
}

impl<IdxLenType, ValLenType, LengthType> DataBlockBuilder<IdxLenType, ValLenType, LengthType> {
    pub fn index_len(self, len: u64) -> DataBlockBuilder<u64, ValLenType, LengthType> {
        DataBlockBuilder {
            index_len : len,
            value_len : self.value_len,
            length    : self.length,
        }
    }

    pub fn value_len(self, len: u64) -> DataBlockBuilder<IdxLenType, u64, LengthType> {
        DataBlockBuilder {
            index_len : self.index_len,
            value_len : len,
            length    : self.length,
        }
    }

    pub fn length(self, len: u64) -> DataBlockBuilder<IdxLenType, ValLenType, u64> {
        DataBlockBuilder {
            index_len : self.index_len,
            value_len : self.value_len,
            length : len,
        }
    }
}

impl DataBlockBuilder<u64, u64, u64> {
    pub fn build(self) -> DataBlock {
        assert_eq!(self.index_len, 1);
        DataBlock {
            index_len : self.index_len,
            value_len : self.value_len,
            length    : self.length,
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LogBlock {
    time    : std::time::SystemTime,
    program : String,
    info    : String,
}

impl LogBlock {
    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Option<Self> {
        let secs = reader.read_u64::<LittleEndian>().unwrap();
        let nanos = reader.read_u32::<LittleEndian>().unwrap();
        println!("secs: {}, nanos: {}", secs, nanos);
        let dur = std::time::Duration::new(secs, nanos);
        let program = read_string_from(reader).unwrap();
        let info = read_string_from(reader).unwrap();
        let log = LogBlock {
            time : std::time::SystemTime::now(),
            program : program,
            info : info,
        };
        Some(log)
    }
}

#[derive(Debug)]
pub struct LogBlockBuilder<ProgType, InfoType> {
    time    : Option<std::time::SystemTime>,
    program : ProgType,
    info    : InfoType,
}

impl LogBlockBuilder<(), ()> {
    pub fn new() -> Self {
        LogBlockBuilder {
            time    : None,
            program : (),
            info    : (),
        }
    }
}

impl LogBlockBuilder<String, String> {
    pub fn build(self) -> LogBlock {
        LogBlock {
            time    : if let Some(t) = self.time { t } else { std::time::SystemTime::now() },
            program : self.program,
            info    : self.info,
        }
    }
}

impl<ProgType, InfoType> LogBlockBuilder<ProgType, InfoType> {
    pub fn program<S: Into<String>>(self, program: S) -> LogBlockBuilder<String, InfoType> {
        LogBlockBuilder {
            time    : self.time,
            program : program.into(),
            info    : self.info,
        }
    }

    pub fn info<S: Into<String>>(self, info: S) -> LogBlockBuilder<ProgType, String> {
        LogBlockBuilder {
            time    : self.time,
            program : self.program,
            info    : info.into(),
        }
    }

    pub fn time(mut self, time: std::time::SystemTime) -> Self {
        self.time = Some(time);
        self
    }
}
