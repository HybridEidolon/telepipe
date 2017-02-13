use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddrV4, SocketAddrV6, Ipv4Addr, Ipv6Addr};

use byteorder::{LittleEndian as LE, BigEndian as BE, ReadBytesExt, WriteBytesExt};

use serial::*;
use serial::util::*;

pub static LOGIN_WELCOME_COPYRIGHT: &'static str = "DreamCast Port Map. Copyright SEGA \
                                                    Enterprises. 1999";

pub static SHIP_WELCOME_COPYRIGHT: &'static str = "DreamCast Lobby Server. Copyright SEGA \
                                                   Enterprises. 1999";

#[derive(Clone, Debug)]
pub struct Welcome {
    pub copyright: String,
    pub server_seed: u32,
    pub client_seed: u32,
}

impl Default for Welcome {
    fn default() -> Welcome {
        Welcome {
            copyright: SHIP_WELCOME_COPYRIGHT.to_string(),
            server_seed: 0,
            client_seed: 0,
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
            client_seed: client_seed,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Redirect4 {
    pub socket_addr: SocketAddrV4,
}

impl Serial for Redirect4 {
    fn serialize<W: Write>(&self, mut w: W) -> io::Result<()> {
        let ip = self.socket_addr.ip().octets();
        try!(w.write_all(&ip));
        try!(self.socket_addr.port().serialize(&mut w));
        try!(0u16.serialize(&mut w));
        Ok(())
    }

    fn deserialize<R: Read>(mut r: R) -> io::Result<Redirect4> {
        let mut ip_buf: [u8; 4] = [0; 4];
        try!(r.read_exact(&mut ip_buf));
        let ip: Ipv4Addr = ip_buf.into();
        let port = try!(Serial::deserialize(&mut r));
        try!(r.read_u16::<LE>());
        let socket_addr = SocketAddrV4::new(ip, port);
        Ok(Redirect4 { socket_addr: socket_addr })
    }
}

impl Default for Redirect4 {
    fn default() -> Redirect4 {
        Redirect4 { socket_addr: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80) }
    }
}

#[derive(Clone, Debug)]
pub struct Redirect6 {
    pub socket_addr: SocketAddrV6,
}

impl Serial for Redirect6 {
    fn serialize<W: Write>(&self, mut w: W) -> io::Result<()> {
        let segments = self.socket_addr.ip().segments();
        for s in &segments[..] {
            try!(w.write_u16::<BE>(*s));
        }
        try!(self.socket_addr.port().serialize(&mut w));
        try!(0u16.serialize(&mut w));
        Ok(())
    }

    fn deserialize<R: Read>(mut r: R) -> io::Result<Redirect6> {
        let mut ip_buf: [u8; 16] = [0; 16];
        try!(r.read_exact(&mut ip_buf));
        let port = try!(Serial::deserialize(&mut r));
        try!(r.read_u16::<LE>());
        let ip: Ipv6Addr = ip_buf.into();
        let socket_addr = SocketAddrV6::new(ip, port, 0, 0xe);
        Ok(Redirect6 { socket_addr: socket_addr })
    }
}

impl Default for Redirect6 {
    fn default() -> Redirect6 {
        Redirect6 {
            socket_addr: SocketAddrV6::new(Ipv4Addr::new(127, 0, 0, 1).to_ipv6_mapped(), 80, 0, 0),
        }
    }
}
