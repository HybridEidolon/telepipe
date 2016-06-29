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
    Welcome(u8, Welcome)
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
            &Msg::Unknown(ref c, ref f, ref b) => {
                code = *c;
                flags = *f;
                try!(cursor.write_all(b));
            }
        }

        let mut buf: Vec<u8> = cursor.into_inner();
        let buf_len = buf.len();
        buf.append(&mut vec![0; round_up_remainder(buf_len as u16, 4) as usize]);

        try!(w.write_u8(code));
        try!(w.write_u8(flags));
        try!(w.write_u16::<LE>(round_up(buf.len() as u16, 4)));
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
            _    => Msg::Unknown(code, flags, buf)
        };

        Ok(ret)
    }
}

#[inline(always)]
fn round_up(val: u16, of: u16) -> u16 {
    if val % of == 0 {
        val
    } else {
        val + of
    }
}

#[inline(always)]
fn round_up_remainder(val: u16, of: u16) -> u16 {
    if val % of == 0 {
        0
    } else {
        of - (val % of)
    }
}


#[cfg(test)]
mod test;
