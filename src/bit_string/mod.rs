pub struct BitString {
    b: u64,
}

impl BitString {
    pub fn new() -> Self {
        BitString { b: 0 }
    }

    pub fn set(&mut self, n: u64) {
        self.b |= 1 << n;
    }

    pub fn count_until(&self, n: u64) -> u64 {
        if n == 0 { return 0 }

        let bits_to_ignore = 64 - n;
        let remaining_bits = self.b << bits_to_ignore;

        u64::from(remaining_bits.count_ones())
    }
}

#[cfg(test)]
mod test;
