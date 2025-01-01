use std::time::SystemTime;

#[derive(Clone, Copy, Debug)]
pub struct Lcg {
    a:    u32,
    c:    u32,
    m:    u32,
    seed: u32,
}

impl Lcg {
    pub fn new(a: u32, c: u32, m: u32, seed: u32) -> Self {
        Self { a,
               c,
               m,
               seed, }
    }

    pub fn with_seed(seed: u32) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    pub fn generate(&mut self) -> u32 {
        (self.a.wrapping_mul(self.seed).wrapping_add(self.c)) % self.m
    }
}

impl Default for Lcg {
    fn default() -> Self {
        let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        Self { a:    1103515245,
               c:    12345,
               m:    2u32.pow(31),
               seed: (seed.as_secs() & 0xFFFFFFFF) as u32, }
    }
}
