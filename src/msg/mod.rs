//! Message structures in PSO.

use std::io;
use std::io::{Read, Write, Cursor};
use serial::Serial;

use byteorder::{LittleEndian as LE, ReadBytesExt, WriteBytesExt};

pub mod common;

use self::common::*;

#[derive(Clone, Debug)]
pub enum Msg {
    Unknown(u8, u8, Vec<u8>),
    Welcome(u8, Welcome),
    Redirect4(Redirect4),
    Redirect6(Redirect6)
}

impl Serial for Msg {
    fn serialize<W: Write>(&self, mut w: W) -> Result<(), io::Error> {
        let code;
        let flags;
        let mut cursor = Cursor::new(Vec::new());

        match self {
            &Msg::Welcome(ref f, ref pl) => {
                code = 0x02;
                flags = *f;
                try!(pl.serialize(&mut cursor));
            },
            &Msg::Redirect4(ref r) => {
                code = 0x19;
                flags = 0;
                try!(r.serialize(&mut cursor));
            },
            &Msg::Redirect6(ref r) => {
                code = 0x19;
                flags = 6;
                try!(r.serialize(&mut cursor));
            }
            &Msg::Unknown(ref c, ref f, ref b) => {
                code = *c;
                flags = *f;
                try!(cursor.write_all(b));
            },
            //_ => unimplemented!()
        }

        let mut buf: Vec<u8> = cursor.into_inner();
        let buf_len = buf.len() + 4;
        buf.append(&mut vec![0; round_up_remainder(buf_len as u16, 4) as usize]);

        try!(w.write_u8(code));
        try!(w.write_u8(flags));
        try!(w.write_u16::<LE>(round_up(buf_len as u16, 4)));
        try!(w.write_all(&buf));

        Ok(())
    }

    fn deserialize<R: Read>(mut r: R) -> Result<Msg, io::Error> {
        let code = try!(r.read_u8());
        let flags = try!(r.read_u8());
        let size = try!(r.read_u16::<LE>());
        let mut buf: Vec<u8> = vec![0; size as usize];
        try!(r.read_exact(&mut buf));

        let ret = match code {
            0x02 => Msg::Welcome(flags, try!(Serial::deserialize(&mut Cursor::new(buf)))),
            0x19 => {
                if flags == 6 {
                    Msg::Redirect6(try!(Serial::deserialize(&mut Cursor::new(buf))))
                } else {
                    Msg::Redirect4(try!(Serial::deserialize(&mut Cursor::new(buf))))
                }
            }
            _    => Msg::Unknown(code, flags, buf)
        };

        Ok(ret)
    }
}

/// Round up a value to a multiple of `of`.
#[inline(always)]
pub fn round_up(val: u16, of: u16) -> u16 {
    val + round_up_remainder(val, of)
}

/// Get the amount required to round up a value to a multiple of `of`.
#[inline(always)]
pub fn round_up_remainder(val: u16, of: u16) -> u16 {
    if val % of == 0 {
        0
    } else {
        of - (val % of)
    }
}


#[cfg(test)]
mod test;
