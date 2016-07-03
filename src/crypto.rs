//! PSO GameCube's cryptography functions.

const NUM_KEYS: usize = 528;

/// An instance of a PSOGC cipher.
pub struct Cipher {
    pos: usize,
    keys: Vec<u32>,
    seed: u32
}

impl Cipher {
    pub fn new(seed: u32) -> Cipher {
        let mut ret = Cipher {
            pos: 0,
            keys: vec![0; NUM_KEYS],
            seed: seed
        };

        ret.initialize();
        ret.print_keys();

        ret
    }

    fn print_keys(&self) {
        println!("ZERO");
        println!("### ###+0000 ###+0001 ###+0002 ###+0003 ###+0004 ###+0005 ###+0006 ###+0007");
        for x in 0..66 {
            println!("{:03} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}", x*8,
                     self.keys[x*8 + 0],
                     self.keys[x*8 + 1],
                     self.keys[x*8 + 2],
                     self.keys[x*8 + 3],
                     self.keys[x*8 + 4],
                     self.keys[x*8 + 5],
                     self.keys[x*8 + 6],
                     self.keys[x*8 + 7]);
        }
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

        debug!("Codecing {} words", buf.len() / 4);

        let mut wordbuf: &mut [u32] = unsafe {
            use std::mem;
            use std::slice;

            let buf_len = buf.len();
            let buf_ptr: *mut u32 = mem::transmute(buf.as_mut_ptr());
            slice::from_raw_parts_mut(buf_ptr, buf_len / 4)
        };

        for w in wordbuf.iter_mut() {
            let key = self.next_key();
            *w = *w ^ key;
        }

        Ok(())
    }

    pub fn encode_word(&mut self, v: u32) -> u32 {
        v ^ self.next_key()
    }

    fn initialize(&mut self) {
        use std::num::Wrapping as W;

        let mut base_key: u32 = 0;
        let mut source1: usize;
        let mut source2: usize;
        let mut source3: usize;

        let mut seed = self.seed;

        for _ in 0..17 {
            for _ in 0..32 {
                seed = (W(seed) * W(0x5D588B65)).0;
                base_key = base_key >> 1;
                seed = (W(seed) + W(1)).0;
                base_key = if seed & 0x80000000 != 0 {
                    base_key | 0x80000000
                } else {
                    base_key & 0x7FFFFFFF
                };
            }
            self.keys[self.pos] = base_key;
            self.pos += 1;
        }
        source1 = 0;
        source2 = 1;
        self.pos -= 1;
        let r = (((W(self.keys[0]) >> 9) ^ (W(self.keys[self.pos]) << 23)) ^ W(self.keys[15])).0;
        self.keys[self.pos] = r;
        source3 = self.pos;
        self.pos += 1;
        while self.pos != 521 {
            // fight me IRL, I am not wasting another 6 hours cleaning this up
            let term1 = W(self.keys[source3]);
            let term2 = W(self.keys[source1]);
            let term3 = W(self.keys[source2]);
            let term4 = term1 << 23;
            let term5 = term4 ^ W(0xFF800000);
            let term6 = term3 & W(0x007FFFFF);
            let term7 = term5 ^ term6;
            let term8 = term1 ^ term7;
            let r = term8.0;
            self.keys[self.pos] = r;
            self.pos += 1;
            source1 += 1;
            source2 += 1;
            source3 += 1;
        }
        self.mix();
        self.mix();
        self.mix();
        self.pos = 520;
    }

    fn mix(&mut self) {
        let mut r0: u32;
        let mut r4: u32;
        let mut r5: usize;
        let mut r6: usize;
        let mut r7: usize;

        self.pos = 0;
        r5 = 0;
        r6 = 489;
        r7 = 0;

        while r6 != 521 {
            r0 = self.keys[r6];
            r6 += 1;
            r4 = self.keys[r5];
            r0 ^= r4;
            self.keys[r5] = r0;
            r5 += 1;
        }

        while r5 != 521 {
            r0 = self.keys[r7];
            r7 += 1;
            r4 = self.keys[r5];
            r0 ^= r4;
            self.keys[r5] = r0;
            r5 += 1;
        }
    }

    pub fn next_key(&mut self) -> u32 {
        self.pos += 1;
        if self.pos == 521 {
            self.mix();
        }
        self.keys[self.pos].to_le()
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
        let seed = 512;
        let mut cipher = Cipher::new(seed);

        assert_eq!(cipher.codec(&mut vec![0; 3]), Err(CodecError::IllegalBufferSize));
        assert_eq!(cipher.codec(&mut Vec::new()), Ok(()));
        assert_eq!(cipher.codec(&mut vec![0; 512]), Ok(()));
    }
}
