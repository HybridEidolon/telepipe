//! PSO GameCube's cryptography functions.

/// An instance of a PSOGC cipher.
pub struct Cipher;

impl Cipher {
    pub fn new(_seed: &[u8]) -> Cipher {
        Cipher
    }

    /// Use the cipher over the given buffer, mutating in-place, and updating
    /// the cipher state in the process. buf must be a multiple of 4.
    pub fn codec(&mut self, buf: &mut [u8]) -> Result<(), CodecError> {
        if buf.len() % 4 != 0 {
            return Err(CodecError::IllegalBufferSize)
        }
        if buf.len() == 0 {
            return Ok(())
        }
        unimplemented!()
    }
}

/// An error in Cipher::codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecError {
    /// The buffer given was either size 0 or not a multiple of 4.
    IllegalBufferSize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_codec_illegal_size() {
        let seed = Vec::new();
        let mut cipher = Cipher::new(&seed);

        assert_eq!(cipher.codec(&mut vec![0; 3]), Err(CodecError::IllegalBufferSize));
        assert_eq!(cipher.codec(&mut Vec::new()), Ok(()));
    }
}