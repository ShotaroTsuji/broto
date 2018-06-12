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
}

#[derive(Serialize,Deserialize,Debug)]
pub struct DataBlock {
    index_size : u64,
    value_size : u64,
    index_type : String,
    value_type : String,
    byteorder  : u32,
    length     : u64,
}

#[derive(Debug)]
pub struct DataBlockBuilder<IndexType,ValueType,LengthType> {
    index_type : IndexType,
    value_type : ValueType,
    length : LengthType,
}

impl DataBlockBuilder<(), (), ()> {
    pub fn new() -> Self {
        DataBlockBuilder {
            index_type : (),
            value_type : (),
            length : (),
        }
    }
}

impl<IndexType,ValueType,LengthType> DataBlockBuilder<IndexType,ValueType,LengthType> {
    pub fn index_type<T: Into<TupleType>>(self, id_type: T) -> DataBlockBuilder<TupleType,ValueType,LengthType> {
        DataBlockBuilder {
            index_type: id_type.into(),
            value_type: self.value_type,
            length    : self.length,
        }
    }

    pub fn value_type<T: Into<TupleType>>(self, val_type: T) -> DataBlockBuilder<IndexType,TupleType,LengthType> {
        DataBlockBuilder {
            index_type : self.index_type,
            value_type : val_type.into(),
            length : self.length,
        }
    }

    pub fn length(self, len: u64) -> DataBlockBuilder<IndexType,ValueType,u64> {
        DataBlockBuilder {
            index_type : self.index_type,
            value_type : self.value_type,
            length : len,
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LogBlock {
    time    : std::time::SystemTime,
    program : String,
    info    : String,
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


#[derive(Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}
