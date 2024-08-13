use std::num::Wrapping;

#[derive(Clone, Debug)]
pub struct Sha1 {
    buffer: Vec<u8>, // this could be a vecdeque
    digest: [u32; Self::digest_ints],
    transforms: usize
}

impl Sha1 {
    const digest_ints: usize = 5;
    const block_ints: usize = 16;
    const block_bytes: usize = Self::block_ints * 4;

    pub fn new() -> Self {
        let mut sha1 = Self { buffer: Vec::default(), digest: [0u32; Self::digest_ints], transforms: 0 };
        sha1.reset();
        sha1
    }

    pub fn finalize_as_str(&mut self) -> String {
        let total_bits = ((self.transforms * Self::block_bytes + self.buffer.len()) * 8) as u64;

        // pad
        self.buffer.push(0x80);
        let old_size = self.buffer.len();
        while self.buffer.len() < Self::block_bytes {
            self.buffer.push(0x00);
        }

        let mut block = [0u32; Self::block_ints];
        self.write_block(&mut block);

        if old_size > (Self::block_bytes - 8) {
            self.transform(&mut block);
            for idx in 0..(Self::block_ints - 2) {
                block[idx] = 0;
            }
        }

        block[Self::block_ints - 1] = total_bits as u32;
        block[Self::block_ints - 2] = (total_bits >> 32) as u32;
        self.transform(&mut block);

        let result = format!("{:x}{:x}{:x}{:x}{:x}", self.digest[0], self.digest[1], self.digest[2], self.digest[3], self.digest[4]);
        self.reset();
        result
    }

    pub fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);

        while self.buffer.len() >= Self::block_bytes {
            let mut block = [0u32; Self::block_ints];
            self.write_block(&mut block);
            self.transform(&mut block);
        }
    }

    /*
        Private
    */

    fn reset(&mut self) {
        self.digest[0] = 0x67452301;
        self.digest[1] = 0xefcdab89;
        self.digest[2] = 0x98badcfe;
        self.digest[3] = 0x10325476;
        self.digest[4] = 0xc3d2e1f0;

        self.transforms = 0;
        self.buffer.clear();
    }

    fn transform(&mut self, block: &mut [u32; Self::block_ints]) {
        let mut a = self.digest[0];
        let mut b = self.digest[1];
        let mut c = self.digest[2];
        let mut d = self.digest[3];
        let mut e = self.digest[4];

        Self::r0(block, a, &mut b, c, d, &mut e,  0);
        Self::r0(block, e, &mut a, b, c, &mut d,  1);
        Self::r0(block, d, &mut e, a, b, &mut c,  2);
        Self::r0(block, c, &mut d, e, a, &mut b,  3);
        Self::r0(block, b, &mut c, d, e, &mut a,  4);
        Self::r0(block, a, &mut b, c, d, &mut e,  5);
        Self::r0(block, e, &mut a, b, c, &mut d,  6);
        Self::r0(block, d, &mut e, a, b, &mut c,  7);
        Self::r0(block, c, &mut d, e, a, &mut b,  8);
        Self::r0(block, b, &mut c, d, e, &mut a,  9);
        Self::r0(block, a, &mut b, c, d, &mut e, 10);
        Self::r0(block, e, &mut a, b, c, &mut d, 11);
        Self::r0(block, d, &mut e, a, b, &mut c, 12);
        Self::r0(block, c, &mut d, e, a, &mut b, 13);
        Self::r0(block, b, &mut c, d, e, &mut a, 14);
        Self::r0(block, a, &mut b, c, d, &mut e, 15);
        Self::r1(block, e, &mut a, b, c, &mut d, 16);
        Self::r1(block, d, &mut e, a, b, &mut c, 17);
        Self::r1(block, c, &mut d, e, a, &mut b, 18);
        Self::r1(block, b, &mut c, d, e, &mut a, 19);
        Self::r2(block, a, &mut b, c, d, &mut e, 20);
        Self::r2(block, e, &mut a, b, c, &mut d, 21);
        Self::r2(block, d, &mut e, a, b, &mut c, 22);
        Self::r2(block, c, &mut d, e, a, &mut b, 23);
        Self::r2(block, b, &mut c, d, e, &mut a, 24);
        Self::r2(block, a, &mut b, c, d, &mut e, 25);
        Self::r2(block, e, &mut a, b, c, &mut d, 26);
        Self::r2(block, d, &mut e, a, b, &mut c, 27);
        Self::r2(block, c, &mut d, e, a, &mut b, 28);
        Self::r2(block, b, &mut c, d, e, &mut a, 29);
        Self::r2(block, a, &mut b, c, d, &mut e, 30);
        Self::r2(block, e, &mut a, b, c, &mut d, 31);
        Self::r2(block, d, &mut e, a, b, &mut c, 32);
        Self::r2(block, c, &mut d, e, a, &mut b, 33);
        Self::r2(block, b, &mut c, d, e, &mut a, 34);
        Self::r2(block, a, &mut b, c, d, &mut e, 35);
        Self::r2(block, e, &mut a, b, c, &mut d, 36);
        Self::r2(block, d, &mut e, a, b, &mut c, 37);
        Self::r2(block, c, &mut d, e, a, &mut b, 38);
        Self::r2(block, b, &mut c, d, e, &mut a, 39);
        Self::r3(block, a, &mut b, c, d, &mut e, 40);
        Self::r3(block, e, &mut a, b, c, &mut d, 41);
        Self::r3(block, d, &mut e, a, b, &mut c, 42);
        Self::r3(block, c, &mut d, e, a, &mut b, 43);
        Self::r3(block, b, &mut c, d, e, &mut a, 44);
        Self::r3(block, a, &mut b, c, d, &mut e, 45);
        Self::r3(block, e, &mut a, b, c, &mut d, 46);
        Self::r3(block, d, &mut e, a, b, &mut c, 47);
        Self::r3(block, c, &mut d, e, a, &mut b, 48);
        Self::r3(block, b, &mut c, d, e, &mut a, 49);
        Self::r3(block, a, &mut b, c, d, &mut e, 50);
        Self::r3(block, e, &mut a, b, c, &mut d, 51);
        Self::r3(block, d, &mut e, a, b, &mut c, 52);
        Self::r3(block, c, &mut d, e, a, &mut b, 53);
        Self::r3(block, b, &mut c, d, e, &mut a, 54);
        Self::r3(block, a, &mut b, c, d, &mut e, 55);
        Self::r3(block, e, &mut a, b, c, &mut d, 56);
        Self::r3(block, d, &mut e, a, b, &mut c, 57);
        Self::r3(block, c, &mut d, e, a, &mut b, 58);
        Self::r3(block, b, &mut c, d, e, &mut a, 59);
        Self::r4(block, a, &mut b, c, d, &mut e, 60);
        Self::r4(block, e, &mut a, b, c, &mut d, 61);
        Self::r4(block, d, &mut e, a, b, &mut c, 62);
        Self::r4(block, c, &mut d, e, a, &mut b, 63);
        Self::r4(block, b, &mut c, d, e, &mut a, 64);
        Self::r4(block, a, &mut b, c, d, &mut e, 65);
        Self::r4(block, e, &mut a, b, c, &mut d, 66);
        Self::r4(block, d, &mut e, a, b, &mut c, 67);
        Self::r4(block, c, &mut d, e, a, &mut b, 68);
        Self::r4(block, b, &mut c, d, e, &mut a, 69);
        Self::r4(block, a, &mut b, c, d, &mut e, 70);
        Self::r4(block, e, &mut a, b, c, &mut d, 71);
        Self::r4(block, d, &mut e, a, b, &mut c, 72);
        Self::r4(block, c, &mut d, e, a, &mut b, 73);
        Self::r4(block, b, &mut c, d, e, &mut a, 74);
        Self::r4(block, a, &mut b, c, d, &mut e, 75);
        Self::r4(block, e, &mut a, b, c, &mut d, 76);
        Self::r4(block, d, &mut e, a, b, &mut c, 77);
        Self::r4(block, c, &mut d, e, a, &mut b, 78);
        Self::r4(block, b, &mut c, d, e, &mut a, 79);
        
        self.digest[0] = self.digest[0].wrapping_add(a);
        self.digest[1] = self.digest[1].wrapping_add(b);
        self.digest[2] = self.digest[2].wrapping_add(c);
        self.digest[3] = self.digest[3].wrapping_add(d);
        self.digest[4] = self.digest[4].wrapping_add(e);
        self.transforms += 1;
    }

    fn write_block(&mut self, block: &mut [u32; Self::block_ints]) {
        for idx in 0..Self::block_ints {
            block[idx] = (self.buffer[4 * idx + 3] as u32 & 0xff) << 0
                | (self.buffer[4 * idx + 2] as u32 & 0xff) << 8
                | (self.buffer[4 * idx + 1] as u32 & 0xff) << 16
                | (self.buffer[4 * idx + 0] as u32 & 0xff) << 24;
        }

        self.buffer.drain(0..Self::block_bytes);
    }

    fn r0(block: &mut[u32; Self::block_ints], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, index: usize) {
        *z = z.wrapping_add(((*w & (x ^ y)) ^ y).wrapping_add(block[index]).wrapping_add(0x5a827999).wrapping_add(Self::roll(v, 5)));
        *w = Self::roll(*w, 30);
    }

    fn r1(block: &mut[u32; Self::block_ints], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, index: usize) {
        *z = z.wrapping_add(((*w & (x ^ y)) ^ y).wrapping_add(Self::blk(block, index)).wrapping_add(0x5a827999).wrapping_add(Self::roll(v, 5)));
        *w = Self::roll(*w, 30);
    }

    fn r2(block: &mut[u32; Self::block_ints], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, index: usize) {
        *z = z.wrapping_add((*w ^ x ^ y).wrapping_add(Self::blk(block, index)).wrapping_add(0x6ed9eba1).wrapping_add(Self::roll(v, 5)));
        *w = Self::roll(*w, 30);
    }

    fn r3(block: &mut[u32; Self::block_ints], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, index: usize) {
        *z = z.wrapping_add((((*w | x) & y) | (*w & x)).wrapping_add(Self::blk(block, index)).wrapping_add(0x8f1bbcdc).wrapping_add(Self::roll(v, 5)));
        *w = Self::roll(*w, 30);
    }

    fn r4(block: &mut[u32; Self::block_ints], v: u32, w: &mut u32, x: u32, y: u32, z: &mut u32, index: usize) {
        *z = z.wrapping_add((*w ^ x ^ y).wrapping_add(Self::blk(block, index)).wrapping_add(0xca62c1d6).wrapping_add(Self::roll(v, 5)));
        *w = Self::roll(*w, 30);
    }

    fn blk(block: &mut [u32; Self::block_ints], index: usize) -> u32 {
        block[index & 15] = Self::roll(block[(index + 13) & 15] ^ block[(index + 8) & 15] ^ block[(index + 2) & 15] ^ block[index & 15], 1);
        block[index & 15]
    }

    fn roll(value: u32, bits: u32) -> u32 {
        (value << bits) | (value >> 32 - bits)
    }
}