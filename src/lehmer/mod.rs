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

    fn from_decimal(mut decimal: u64, n: usize) -> Self {
        let mut vec: Vec<u64> = Vec::with_capacity(n);
        let mut product: u64 = 1;

        vec.resize(n, 0);

        for (iteration, index) in (1..n).zip((0..n-1).rev()) {
            product *= iteration as u64;
            decimal /= product;
            vec[index] = decimal % (iteration as u64 + 1);
        }

        Self::new(vec)
    }

    fn to_permutation(self) -> Vec<u64> {
        let n = self.vec.len() as u64;

        let mut sequence: Vec<u64> = (0..n).collect();
        let mut permutation: Vec<u64> = Vec::with_capacity(n as usize);

        for digit in self.vec {
            let element = sequence.remove(digit as usize);
            permutation.push(element);
        }

        permutation
    }

    fn to_decimal(self) -> u64 {
        let mut product: u64 = 1;
        let mut decimal: u64 = 0;

        for (i, d) in self.vec.iter().rev().enumerate().skip(1) {
            product *= i as u64;
            decimal += d * product;
        }

        decimal
    }
}

#[cfg(test)]
mod test;
