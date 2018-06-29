use std;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use error::{WriteError, ReadError};


fn read_string_from<R: std::io::Read>(reader: &mut R) -> Result<String, ReadError> {
    let mut v = Vec::new();
    let len = reader.read_u64::<LittleEndian>()?;
    for _ in 0..len {
        let c = reader.read_u8()?;
        v.push(c);
    }
    String::from_utf8(v).map_err(|e| ReadError::FromUtf8(e))
}

fn write_string_into<W: std::io::Write>(string: &String, writer: &mut W) -> Result<(), WriteError> {
    writer.write_u64::<LittleEndian>(string.len() as u64)?;
    for c in string.bytes() {
        writer.write_u8(c)?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct Header {
    magic_number    : [u8; 8],
    header_size     : u64,
    major_version   : u32,
    minor_version   : u32,
    file_size       : u64,
}

impl Header {
    pub fn new(file_size: u64) -> Header {
        Header {
            magic_number  : Header::clone_magic(),
            header_size   : std::mem::size_of::<Header>() as u64,
            major_version : 0,
            minor_version : 1,
            file_size     : file_size,
        }
    }

    pub fn clone_magic() -> [u8; 8] {
        let mut magic = [0; 8];
        magic.clone_from_slice("tsbinfmt".as_bytes());
        magic
    }

    pub fn check_magic(input: &[u8]) -> bool {
        let magic = "tsbinfmt".as_bytes();
        magic.iter().zip(input.iter()).all(|(&x, &y)| x == y)
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Result<Header, ReadError> {
        let mut magic: [u8; 8] = [0; 8];
        let result = reader.read(&mut magic);
        match result {
            Ok(n) if n < 8 => { return Err(ReadError::EndOfFile); },
            Err(e) => { return Err(ReadError::Io(e)); },
            _ => {},
        }
        if Header::check_magic(&magic) == false {
            return Err(ReadError::Magic);
        }
        let header_size = reader.read_u64::<LittleEndian>()?;
        let major_version = reader.read_u32::<LittleEndian>()?;
        let minor_version = reader.read_u32::<LittleEndian>()?;
        let file_size = reader.read_u64::<LittleEndian>()?;
        let hd = Header {
            magic_number : magic,
            header_size  : header_size,
            major_version: major_version,
            minor_version: minor_version,
            file_size    : file_size,
        };
        Ok(hd)
    }

    pub fn write_into<W: std::io::Write>(&self, writer: &mut W) -> Result<(), WriteError> {
        let result = writer.write(&self.magic_number);
        match result {
            Ok(n) if n < 8 => { return Err(WriteError::EndOfFile); },
            Err(e) => { return Err(e.into()); },
            _ => {},
        }
        writer.write_u64::<LittleEndian>(self.header_size)?;
        writer.write_u32::<LittleEndian>(self.major_version)?;
        writer.write_u32::<LittleEndian>(self.minor_version)?;
        writer.write_u64::<LittleEndian>(self.file_size)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct BlockHeader {
    magic : [u8; 8],
    name  : String,
    size  : u64,
}

impl BlockHeader {
    pub fn new<S: Into<String>>(name: S, size: u64) -> Self {
        BlockHeader {
            magic : Self::clone_magic(),
            name  : name.into(),
            size  : size,
        }
    }

    pub fn clone_name(&self) -> String {
        String::from(self.name.clone())
    }

    pub fn clone_magic() -> [u8; 8] {
        let mut magic = [0; 8];
        magic.clone_from_slice("block   ".as_bytes());
        magic
    }

    pub fn check_magic(input: &[u8]) -> bool {
        let magic = "block   ".as_bytes();
        magic.iter().zip(input.iter()).all(|(&x, &y)| x == y)
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        let mut magic: [u8; 8] = [0; 8];
        let result = reader.read(&mut magic);
        match result {
            Ok(n) if n < 8 => { return Err(ReadError::EndOfFile); },
            Err(e) => { return Err(e.into()); },
            _ => {},
        }
        if Self::check_magic(&magic) == false {
            return Err(ReadError::Magic);
        }
        let name = read_string_from(reader)?;
        let size = reader.read_u64::<LittleEndian>()?;
        let hd = BlockHeader {
            magic : magic,
            name  : name,
            size  : size,
        };
        Ok(hd)
    }

    pub fn write_into<W: std::io::Write>(&self, writer: &mut W) -> Result<(), WriteError> {
        let result = writer.write(&self.magic);
        match result {
            Ok(n) if n < 8 => { return Err(WriteError::EndOfFile); },
            Err(e) => { return Err(e.into()); },
            _ => {},
        }
        write_string_into(&self.name, writer)?;
        writer.write_u64::<LittleEndian>(self.size)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FloatTSBlock {
    index_len  : u64,
    value_len  : u64,
    length     : u64,
}

impl FloatTSBlock {
    pub fn index_len(&self) -> u64 {
        self.index_len
    }

    pub fn value_len(&self) -> u64 {
        self.value_len
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn size(&self) -> usize {
        8 + 8 + 8
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        let index_len = reader.read_u64::<LittleEndian>()?;
        let value_len = reader.read_u64::<LittleEndian>()?;
        let length = reader.read_u64::<LittleEndian>()?;
        Ok(FloatTSBlock {
            index_len: index_len,
            value_len: value_len,
            length: length,
        })
    }

    pub fn write_into<W: std::io::Write>(&self, writer: &mut W) -> Result<(), WriteError> {
        writer.write_u64::<LittleEndian>(self.index_len)?;
        writer.write_u64::<LittleEndian>(self.value_len)?;
        writer.write_u64::<LittleEndian>(self.length)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FloatTSBlockBuilder<IdxLenType,ValLenType,LengthType> {
    index_len : IdxLenType,
    value_len : ValLenType,
    length    : LengthType,
}

impl FloatTSBlockBuilder<(), (), ()> {
    pub fn new() -> Self {
        FloatTSBlockBuilder {
            index_len : (),
            value_len : (),
            length : (),
        }
    }
}

impl<IdxLenType, ValLenType, LengthType> FloatTSBlockBuilder<IdxLenType, ValLenType, LengthType> {
    pub fn index_len(self, len: u64) -> FloatTSBlockBuilder<u64, ValLenType, LengthType> {
        FloatTSBlockBuilder {
            index_len : len,
            value_len : self.value_len,
            length    : self.length,
        }
    }

    pub fn value_len(self, len: u64) -> FloatTSBlockBuilder<IdxLenType, u64, LengthType> {
        FloatTSBlockBuilder {
            index_len : self.index_len,
            value_len : len,
            length    : self.length,
        }
    }

    pub fn length(self, len: u64) -> FloatTSBlockBuilder<IdxLenType, ValLenType, u64> {
        FloatTSBlockBuilder {
            index_len : self.index_len,
            value_len : self.value_len,
            length : len,
        }
    }
}

impl FloatTSBlockBuilder<u64, u64, u64> {
    pub fn build(self) -> FloatTSBlock {
        assert_eq!(self.index_len, 1);
        FloatTSBlock {
            index_len : self.index_len,
            value_len : self.value_len,
            length    : self.length,
        }
    }
}

#[derive(Debug)]
pub struct LogBlock {
    time    : std::time::Duration,
    program : String,
    info    : String,
}

impl LogBlock {
    pub fn program(&self) -> String {
        self.program.clone()
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }

    pub fn size(&self) -> usize {
        8 + 4 + (8 + self.program.len()) + (8 + self.info.len())
    }

    pub fn read_from<R: std::io::Read>(reader: &mut R) -> Result<Self, ReadError> {
        let secs = reader.read_u64::<LittleEndian>()?;
        let nanos = reader.read_u32::<LittleEndian>()?;
        let dur = std::time::Duration::new(secs, nanos);
        let program = read_string_from(reader)?;
        let info = read_string_from(reader)?;
        let log = LogBlock {
            time : dur,
            program : program,
            info : info,
        };
        Ok(log)
    }

    pub fn write_into<W: std::io::Write>(&self, writer: &mut W) -> Result<(), WriteError> {
        writer.write_u64::<LittleEndian>(self.time.as_secs())?;
        writer.write_u32::<LittleEndian>(self.time.subsec_nanos())?;
        write_string_into(&self.program, writer)?;
        write_string_into(&self.info, writer)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct LogBlockBuilder<ProgType, InfoType> {
    time    : Option<std::time::Duration>,
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
            time    : if let Some(t) = self.time { t } else { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap() },
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

    pub fn time(mut self, time: std::time::Duration) -> Self {
        self.time = Some(time);
        self
    }
}
