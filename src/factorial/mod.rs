use bit_string::BitString;

struct Factorial {
    vec: Vec<u64>,
}

impl Factorial {
    fn from_permutation(mut vec: Vec<u64>) -> Self {
        let mut bit_string = BitString::new();

        for k in &mut vec {
            bit_string.set(*k);
            *k -= bit_string.count_until(*k);
        }

        Factorial { vec }
    }
}

#[cfg(test)]
mod test;
