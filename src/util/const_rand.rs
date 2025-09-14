pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub const fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
