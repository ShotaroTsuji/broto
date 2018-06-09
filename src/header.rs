use tupletype::TupleType;
use std;

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
}

#[derive(Debug)]
pub struct BlockHeader {
    magic : [u8; 8],
    name  : String,
    size  : u64,
}

#[derive(Debug)]
pub struct DataBlock {
    index_size : u64,
    value_size : u64,
    index_type : String,
    value_type : String,
    byteorder  : u32,
    length     : u64,
}

#[derive(Debug)]
pub struct LogBlock {
    time    : std::time::SystemTime,
    program : String,
    info    : String,
}


#[derive(Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}
