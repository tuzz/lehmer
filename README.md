## Lehmer

![Travis](https://travis-ci.org/tuzz/lehmer.svg?branch=master)

Lehmer is a
[Rust crate](https://crates.io/crates/lehmer)
for converting between permutation vectors,
[Lehmer codes](https://en.wikipedia.org/wiki/Lehmer_code)
and their
[decimal representations](https://en.wikipedia.org/wiki/Factorial_number_system#Permutations).
It is designed to run as quickly as possible
and as a result, doesn't do any error handling.

This implementation is based on the 'Mapping Permutations to Integers' section
of [this paper](https://www.cs.helsinki.fi/u/bmmalone/heuristic-search-fall-2013/Korf2008.pdf).
It doesn't currently implement the linear time speedup for the reverse mapping.
This requires a lookup table to be precomputed and complicates things a bit.

### Usage

```rust
extern crate lehmer;

use lehmer::Lehmer;

fn main() {
    // Compute the Lehmer code for a permutation:
    let lehmer = Lehmer::from_permutation(vec![1, 0, 4, 3, 2]);

    assert_eq!(vec![1, 0, 2, 1, 0], lehmer.code);
    assert_eq!(29, lehmer.to_decimal());

    // Compute the Lehmer code for a decimal (requires the permutation length)
    let another = Lehmer::from_decimal(29, 5);

    assert_eq!(vec![1, 0, 2, 1, 0], another.code);
    assert_eq!(vec![1, 0, 4, 3, 2], another.to_permutation());
}
```

Lehmer supports permutations up to 21 elements in length whose decimal
representations are less than or equal to `<u64>::max_value()`. Rust will
`panic` for larger values.

Additionally, permutations must be vectors containing sequential integers
starting from 0 (in any order), e.g. `[1, 0, 4, 3, 2]`. Lehmer will either
`panic` or produce incorrect results for other vectors.

Note: The above conditions still mean Lehmer is safe as per Rust's definitions.
It doesn't use any `unsafe` features and memory remains consistent in the case
of a `panic`.

### Benchmarks

Benchmarks can be run with `cargo bench`:

```
test benchmark_from_decimal     ... bench:         263 ns/iter (+/- 7)
test benchmark_from_permutation ... bench:          77 ns/iter (+/- 2)
test benchmark_to_decimal       ... bench:          42 ns/iter (+/- 5)
test benchmark_to_permutation   ... bench:         137 ns/iter (+/- 9)
```

e.g. `Lehmer::from_permutation` runs at ~13 million iterations per second.

### Tests

Tests can be run with `cargo test`. Unit tests are in files next to their
modules and integration tests are in `tests/`.

### Contributions

Contributions are welcome. Please test/benchmark your changes and open a PR.
