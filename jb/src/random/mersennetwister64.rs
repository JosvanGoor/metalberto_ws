
// untested

#[derive(Copy, Clone, Debug)]
pub struct MersenneTwister64 {
    seed: u64,
    index: usize,
    extracted: u64,
    state: [u64; Self::n]
}

impl MersenneTwister64 {
    const a: u64 = 0xB5026F5AA96619E9; // coefficients of rational normal form twist matrix
    const b: u64 = 0x71D67FFFEDA60000; // tempering bitmask
    const c: u64 = 0xFFF7EEE000000000; // tempering bitmask
    const d: u64 = 0x5555555555555555; // additional shift/mask
    const f: u64 = 6364136223846793005;
    const l: u64 = 43; // additional shift/mask
    const m: usize = 156; // middle word
    const n: usize = 312; // degree of recurrence
    const r: u64 = 31; // separation pouint64_t
    const s: u64 = 17; // tempering bit shift
    const t: u64 = 37; // tempering but shift
    const u: u64 = 29; // additional shift/mask
    const w: u64 = 64; // word size
    const upper_mask: u64 = (u64::MAX << Self::r);
    const lower_mask: u64 = !Self::upper_mask;

    pub fn new(seed: u64) -> Self {
        let mut mt64 = MersenneTwister64{seed: seed, index: 0, extracted: 0, state: [0u64; Self::n]};
        mt64.state[0] = seed;
        for idx in 1..Self::n {
            let mut x = mt64.state[idx - 1];
            x ^= x >> (Self::w - 2);
            x *= Self::f;
            x += (idx % Self::n) as u64;
            mt64.state[idx] = x;
        }

        mt64.twist();
        mt64
    }

    pub fn next(&mut self) -> u64 {
        if self.index == Self::n {
            self.twist();
        }

        let mut y = self.state[self.index];
        y = y ^ ((y >> Self::u) & Self::d);
        y = y ^ ((y << Self::s) & Self::b);
        y = y ^ ((y << Self::t) & Self::c);
        y = y ^ (y >> Self::l);

        self.extracted += 1;
        self.index += 1;
        y
    }

    pub fn discard(&mut self, mut steps: usize) {
        // take the next few until we hit a twist barrier
        while self.index != 0 && steps > 0 {
            self.next();
            steps -= 1;
        }

        // twist takes Self::n steps
        while steps >= Self::n {
            self.twist();
            steps -= Self::n;
        }

        // remove any leftover steps
        while steps > 0 {
            self.next();
            steps -= 1;
        }
    }

    // private

    pub fn twist(&mut self) {
        for idx in 0..(Self::n - Self::m) {
            let y = (self.state[idx] & Self::upper_mask) | (self.state[idx + 1] & Self::lower_mask);
            self.state[idx] = self.state[idx + Self::m] ^ (y >> 1) ^ if y & 0x01 == 1 { Self::a } else { 0 };
        }

        for idx in (Self::n - Self::m)..(Self::n - 1) {
            let y = self.state[idx] & Self::upper_mask | self.state[idx + 1] & Self::lower_mask;
            self.state[idx] = self.state[(idx + Self::m) - Self::n] ^ (y >> 1) ^ if y & 0x01 == 1 { Self::a } else { 0 };
        }

        let y = (self.state[Self::n - 1] & Self::upper_mask) | (self.state[0] & Self::lower_mask);
        self.state[Self::n - 1] = (self.state[Self::m - 1] & (y >> 1)) ^ if y & 0x01 == 1 { Self::a } else { 0 };
        self.index = 0;
    }
}