use std;
use std::io;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use error::{Result, Error};


fn read_string_from<R: io::Read>(reader: &mut R) -> Result<String> {
    let mut v = Vec::new();
    let len = reader.read_u64::<LittleEndian>()?;
    for _ in 0..len {
        let c = reader.read_u8()?;
        v.push(c);
    }
    String::from_utf8(v).map_err(|e| Error::FromUtf8(e))
}

fn write_string_into<W: io::Write>(string: &String, writer: &mut W) -> Result<()> {
    writer.write_u64::<LittleEndian>(string.len() as u64)?;
    for c in string.bytes() {
        writer.write_u8(c)?;
    }
    Ok(())
}

#[derive(Debug,PartialEq)]
pub struct Header {
    magic_number    : [u8; 8],
    header_size     : u64,
    major_version   : u32,
    minor_version   : u32,
    reserved0       : u64,
    reserved1       : u64,
    reserved2       : u64,
    reserved3       : u64,
}

impl Header {
    pub fn new() -> Header {
        Header {
            magic_number  : Header::clone_magic(),
            header_size   : std::mem::size_of::<Header>() as u64,
            major_version : 0,
            minor_version : 1,
            reserved0     : 0,
            reserved1     : 0,
            reserved2     : 0,
            reserved3     : 0,
        }
    }

    pub fn clone_magic() -> [u8; 8] {
        let mut magic = [0; 8];
        magic.clone_from_slice("brotofmt".as_bytes());
        magic
    }

    pub fn check_magic(input: &[u8]) -> bool {
        let magic = "brotofmt".as_bytes();
        magic.iter().zip(input.iter()).all(|(&x, &y)| x == y)
    }

    pub fn read_from<R: io::Read>(reader: &mut R) -> Result<Self> {
        let mut magic: [u8; 8] = [0; 8];
        let result = reader.read(&mut magic);
        match result {
            Ok(n) if n < 8 => { return Err(Error::EndOfFile); },
            Err(e) => { return Err(Error::Io(e)); },
            _ => {},
        }
        if Header::check_magic(&magic) == false {
            return Err(Error::Magic);
        }
        let header_size = reader.read_u64::<LittleEndian>()?;
        let major_version = reader.read_u32::<LittleEndian>()?;
        let minor_version = reader.read_u32::<LittleEndian>()?;
        let reserved0 = reader.read_u64::<LittleEndian>()?;
        let reserved1 = reader.read_u64::<LittleEndian>()?;
        let reserved2 = reader.read_u64::<LittleEndian>()?;
        let reserved3 = reader.read_u64::<LittleEndian>()?;
        let hd = Header {
            magic_number : magic,
            header_size  : header_size,
            major_version: major_version,
            minor_version: minor_version,
            reserved0    : reserved0,
            reserved1    : reserved1,
            reserved2    : reserved2,
            reserved3    : reserved3,
        };
        Ok(hd)
    }

    pub fn write_into<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        let result = writer.write(&self.magic_number);
        match result {
            Ok(n) if n < 8 => { return Err(Error::EndOfFile); },
            Err(e) => { return Err(e.into()); },
            _ => {},
        }
        writer.write_u64::<LittleEndian>(self.header_size)?;
        writer.write_u32::<LittleEndian>(self.major_version)?;
        writer.write_u32::<LittleEndian>(self.minor_version)?;
        writer.write_u64::<LittleEndian>(self.reserved0)?;
        writer.write_u64::<LittleEndian>(self.reserved1)?;
        writer.write_u64::<LittleEndian>(self.reserved2)?;
        writer.write_u64::<LittleEndian>(self.reserved3)?;
        Ok(())
    }
}

#[derive(Debug,PartialEq)]
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

    pub fn read_from<R: io::Read>(reader: &mut R) -> Result<Self> {
        let mut magic: [u8; 8] = [0; 8];
        let result = reader.read(&mut magic);
        match result {
            Ok(n) if n < 8 => { return Err(Error::EndOfFile); },
            Err(e) => { return Err(e.into()); },
            _ => {},
        }
        if Self::check_magic(&magic) == false {
            return Err(Error::Magic);
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

    pub fn write_into<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        let result = writer.write(&self.magic);
        match result {
            Ok(n) if n < 8 => { return Err(Error::EndOfFile); },
            Err(e) => { return Err(e.into()); },
            _ => {},
        }
        write_string_into(&self.name, writer)?;
        writer.write_u64::<LittleEndian>(self.size)?;
        Ok(())
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct F64TSBlock {
    index_len  : u64,
    value_len  : u64,
    length     : Option<u64>,
}

impl F64TSBlock {
    pub fn index_len(&self) -> u64 {
        self.index_len
    }

    pub fn value_len(&self) -> u64 {
        self.value_len
    }

    pub fn length(&self) -> Option<u64> {
        self.length
    }

    pub fn set_length(&mut self, len: u64) {
        self.length = Some(len);
    }

    pub fn size(&self) -> usize {
        8 + 8 + 8
    }

    pub fn read_from<R: io::Read>(reader: &mut R) -> Result<Self> {
        let index_len = reader.read_u64::<LittleEndian>()?;
        let value_len = reader.read_u64::<LittleEndian>()?;
        let length = reader.read_u64::<LittleEndian>()?;
        Ok(F64TSBlock {
            index_len: index_len,
            value_len: value_len,
            length: Some(length),
        })
    }

    pub fn write_into<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u64::<LittleEndian>(self.index_len)?;
        writer.write_u64::<LittleEndian>(self.value_len)?;
        writer.write_u64::<LittleEndian>(self.length.unwrap_or(0))?;
        Ok(())
    }
}

#[derive(Debug,PartialEq)]
pub struct F64TSBlockBuilder<IdxLenType,ValLenType> {
    index_len : IdxLenType,
    value_len : ValLenType,
    length    : Option<u64>,
}

impl F64TSBlockBuilder<(), ()> {
    pub fn new() -> Self {
        F64TSBlockBuilder {
            index_len : (),
            value_len : (),
            length : None,
        }
    }
}

impl<IdxLenType, ValLenType> F64TSBlockBuilder<IdxLenType, ValLenType> {
    pub fn index_len(self, len: u64) -> F64TSBlockBuilder<u64, ValLenType> {
        F64TSBlockBuilder {
            index_len : len,
            value_len : self.value_len,
            length    : self.length,
        }
    }

    pub fn value_len(self, len: u64) -> F64TSBlockBuilder<IdxLenType, u64> {
        F64TSBlockBuilder {
            index_len : self.index_len,
            value_len : len,
            length    : self.length,
        }
    }

    pub fn length(self, len: u64) -> Self {
        F64TSBlockBuilder {
            index_len : self.index_len,
            value_len : self.value_len,
            length : Some(len),
        }
    }
}

impl F64TSBlockBuilder<u64, u64> {
    pub fn build(self) -> F64TSBlock {
        assert_eq!(self.index_len, 1);
        F64TSBlock {
            index_len : self.index_len,
            value_len : self.value_len,
            length    : self.length,
        }
    }
}

#[derive(Debug,PartialEq)]
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

    pub fn time(&self) -> std::time::Duration {
        self.time.clone()
    }

    pub fn size(&self) -> usize {
        8 + 4 + (8 + self.program.len()) + (8 + self.info.len())
    }

    pub fn read_from<R: io::Read>(reader: &mut R) -> Result<Self> {
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

    pub fn write_into<W: io::Write>(&self, writer: &mut W) -> Result<()> {
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
            time    : self.time.unwrap_or(std::time::SystemTime::now()
                                          .duration_since(std::time::UNIX_EPOCH)
                                          .unwrap()),
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
