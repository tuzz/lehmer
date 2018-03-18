mod bit_string;

use bit_string::BitString;

pub struct Lehmer {
    pub code: Vec<u64>,
}

impl Lehmer {
    pub fn from_permutation(mut vec: Vec<u64>) -> Self {
        let mut bit_string = BitString::new();

        for k in &mut vec {
            bit_string.set(*k);
            *k -= bit_string.count_until(*k);
        }

        Self { code: vec }
    }

    pub fn from_decimal(decimal: u64, n: usize) -> Self {
        let mut code: Vec<u64> = Vec::with_capacity(n);
        code.resize(n, 0);

        let mut product: u64 = 1;
        let mut iteration: u64 = 1;

        for index in (0..n-1).rev() {
            product *= iteration;
            iteration += 1;

            let divisor = decimal / product;
            let remainder = divisor % iteration;

            code[index] = remainder;
        }

        Self { code }
    }

    pub fn to_permutation(mut self) -> Vec<u64> {
        let n = self.code.len() as u64;
        let mut sequence: Vec<u64> = (0..n).collect();

        for d in &mut self.code {
            *d = sequence.remove(*d as usize);
        }

        self.code
    }

    pub fn to_decimal(self) -> u64 {
        let mut product: u64 = 1;
        let mut decimal: u64 = 0;

        for (i, d) in self.code.iter().rev().enumerate().skip(1) {
            product *= i as u64;
            decimal += d * product;
        }

        decimal
    }

    pub fn max_value(n: usize) -> u64 {
        let mut product: u64 = 1;

        for i in (0..n+1).skip(1) {
            product *= i as u64;
        }

        product - 1
    }
}

#[cfg(test)]
mod test;
