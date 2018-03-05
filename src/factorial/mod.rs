use bit_string::BitString;

struct Factorial {
    digits: Vec<u64>,
}

impl Factorial {
    fn from_permutation(vec: Vec<u64>) -> Self {
        let mut digits = Vec::with_capacity(vec.len());
        let mut bit_string = BitString::new();

        for k in vec {
            digits.push(k - bit_string.count_until(k + 1));
            bit_string.set(k);
        }

        Factorial { digits }
    }
}

#[cfg(test)]
mod test;
