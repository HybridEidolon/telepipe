//! Trait for serializing types into `Read`/`Write` streams.

use std::io;
use std::io::{Read, Write};

use byteorder::{BigEndian as BE, ReadBytesExt, WriteBytesExt};

pub mod util;

/// A type that may be hand-serialized into Read and Write.
pub trait Serial: Sized {
    /// Serialize into a Write.
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error>;
    /// Deserialize from a Read.
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error>;
}

// NOTE: We're using Big Endian here because that's how the PSO streams are.

impl Serial for u8 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_u8(*self));
        Ok(())
    }

    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_u8());
        Ok(v)
    }
}

impl Serial for i8 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_i8(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_i8());
        Ok(v)
    }
}

impl Serial for u16 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_u16::<BE>(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_u16::<BE>());
        Ok(v)
    }
}

impl Serial for i16 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_i16::<BE>(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_i16::<BE>());
        Ok(v)
    }
}

impl Serial for u32 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_u32::<BE>(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_u32::<BE>());
        Ok(v)
    }
}

impl Serial for i32 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_i32::<BE>(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_i32::<BE>());
        Ok(v)
    }
}

impl Serial for u64 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_u64::<BE>(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_u64::<BE>());
        Ok(v)
    }
}

impl Serial for i64 {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_i64::<BE>(*self));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_i64::<BE>());
        Ok(v)
    }
}

impl Serial for bool {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        try!(write.write_u8(if *self {1} else {0}));
        Ok(())
    }
    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let v = try!(read.read_u8());
        Ok(v != 0)
    }
}