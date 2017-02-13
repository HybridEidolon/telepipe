//! Login related message structures.

use std::io;
use std::io::{Read, Write};

use serial::*;

#[derive(Clone, Debug, Default)]
pub struct HlCheck {
    pub serial: [u8; 8],
    pub access_key: [u8; 12],
    pub version: u8,
    pub serial2: [u8; 8],
    pub access_key2: [u8; 12],
    pub password: [u8; 16],
}

impl Serial for HlCheck {
    #[inline(always)]
    fn serialize<W: Write>(&self, mut write: W) -> Result<(), io::Error> {
        write.write_all(&[0; 32][..])?;
        write.write_all(&self.serial[..])?;
        write.write_all(&[0; 8][..])?;
        write.write_all(&self.access_key[..])?;
        write.write_all(&[0; 12][..])?;
        self.version.serialize(&mut write)?;
        write.write_all(&[0; 3][..])?;
        write.write_all(&self.serial2[..])?;
        write.write_all(&[0; 40][..])?;
        write.write_all(&self.access_key2[..])?;
        write.write_all(&[0; 36][..])?;
        write.write_all(&self.password[..])?;
        write.write_all(&[0; 32][..])?;
        Ok(())
    }

    #[inline(always)]
    fn deserialize<R: Read>(mut read: R) -> Result<Self, io::Error> {
        let mut padding = [0; 40];

        let mut serial = [0; 8];
        let mut access_key = [0; 12];
        let version;
        let mut serial2 = [0; 8];
        let mut access_key2 = [0; 12];
        let mut password = [0; 16];

        read.read_exact(&mut padding[..32])?;
        read.read_exact(&mut serial[..])?;
        read.read_exact(&mut padding[..8])?;
        read.read_exact(&mut access_key[..])?;
        read.read_exact(&mut padding[..12])?;
        version = Serial::deserialize(&mut read)?;
        read.read_exact(&mut padding[..3])?;
        read.read_exact(&mut serial2[..])?;
        read.read_exact(&mut padding[..40])?;
        read.read_exact(&mut access_key2[..])?;
        read.read_exact(&mut padding[..36])?;
        read.read_exact(&mut password[..])?;
        read.read_exact(&mut padding[..32])?;
        Ok(HlCheck {
            serial: serial,
            access_key: access_key,
            version: version,
            serial2: serial2,
            access_key2: access_key2,
            password: password,
        })
    }
}
