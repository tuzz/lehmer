pub struct BitString {
    b: u32,
}

impl BitString {
    pub fn new() -> Self {
        BitString { b: 0 }
    }

    pub fn set(&mut self, n: u8) {
        self.b |= 1 << n;
    }

    pub fn count_until(&self, n: u8) -> u8 {
        if n == 0 { return 0 }

        let bits_to_ignore = 32 - n;
        let remaining_bits = self.b << bits_to_ignore;

        remaining_bits.count_ones() as u8
    }
}

#[cfg(test)]
mod test;
