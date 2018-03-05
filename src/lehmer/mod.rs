use bit_string::BitString;

struct Lehmer {
    vec: Vec<u64>,
}

impl Lehmer {
    fn new(vec: Vec<u64>) -> Self {
        Self { vec }
    }

    fn from_permutation(mut vec: Vec<u64>) -> Self {
        let mut bit_string = BitString::new();

        for k in &mut vec {
            bit_string.set(*k);
            *k -= bit_string.count_until(*k);
        }

        Self::new(vec)
    }

    fn to_decimal(self) -> u64 {
        let mut product: u64 = 1;
        let mut sum: u64 = 0;

        for (i, d) in self.vec.iter().rev().enumerate().skip(1) {
            product *= i as u64;
            sum += d * product;
        }

        sum
    }
}

#[cfg(test)]
mod test;
