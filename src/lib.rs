mod bit_string;

use bit_string::BitString;

pub struct Lehmer {
    pub code: Vec<u8>,
}

impl Lehmer {
    pub fn from_permutation(slice: &[u8]) -> Self {
        let mut bit_string = BitString::new();

        let code = slice.iter().map(|k| {
            bit_string.set(*k);
            *k - bit_string.count_until(*k)
        }).collect();

        Self { code }
    }

    pub fn from_decimal(decimal: usize, n: usize) -> Self {
        let mut code: Vec<u8> = Vec::with_capacity(n);
        code.resize(n, 0);

        let mut product: usize = 1;
        let mut iteration: usize = 1;

        for index in (0..n.saturating_sub(1)).rev() {
            product *= iteration;
            iteration += 1;

            let divisor = decimal / product;
            let remainder = divisor % iteration;

            code[index] = remainder as u8;
        }

        Self { code }
    }

    pub fn to_permutation(mut self) -> Vec<u8> {
        let n = self.code.len() as u8;
        let mut sequence: Vec<u8> = (0..n).collect();

        for d in &mut self.code {
            *d = sequence.remove(*d as usize);
        }

        self.code
    }

    pub fn to_decimal(self) -> usize {
        let mut product: usize = 1;
        let mut decimal: usize = 0;

        for (i, d) in self.code.iter().rev().enumerate().skip(1) {
            product *= i as usize;
            decimal += *d as usize * product;
        }

        decimal
    }

    pub fn max_value(n: usize) -> usize {
        let mut product: usize = 1;

        for i in (0..n+1).skip(1) {
            product *= i as usize;
        }

        product - 1
    }
}

#[cfg(test)]
mod test;
