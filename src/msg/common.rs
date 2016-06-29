use std::io;
use std::io::{Read, Write};

use serial::*;
use serial::util::*;

pub static WELCOME_COPYRIGHT: &'static str =
    "DreamCast Port Map. Copyright SEGA Enterprises. 1999";

#[derive(Clone, Debug)]
pub struct Welcome {
    pub copyright: String,
    pub server_seed: u32,
    pub client_seed: u32
}

impl Default for Welcome {
    fn default() -> Welcome {
        Welcome {
            copyright: WELCOME_COPYRIGHT.to_string(),
            server_seed: 0,
            client_seed: 0
        }
    }
}

impl Serial for Welcome {
    fn serialize<W: Write>(&self, mut w: W) -> io::Result<()> {
        try!(write_ascii_len(&self.copyright, 0x40, &mut w));
        try!(self.server_seed.serialize(&mut w));
        try!(self.client_seed.serialize(&mut w));
        Ok(())
    }

    fn deserialize<R: Read>(mut r: R) -> io::Result<Welcome> {
        let copyright = try!(read_ascii_len(0x40, &mut r));
        let server_seed = try!(Serial::deserialize(&mut r));
        let client_seed = try!(Serial::deserialize(&mut r));
        Ok(Welcome {
            copyright: copyright,
            server_seed: server_seed,
            client_seed: client_seed
        })
    }
}
