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
It doesn't implement the linear time speedup as the speed gains are only ~10%
and require precomputing a lookup table.

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

    // Compute the maximum decimal value for a permutation of five elements
    let max = Lehmer::max_value(5);
    assert_eq!(119, max);
}
```

Lehmer supports permutations up to 20 elements in length. The behaviour is not
specified for permutations longer than this and will likely cause a panic. No
unsafe operations are used, though.

Additionally, permutations must be vectors containing sequential integers
starting from 0 (in any order), e.g. `[1, 0, 4, 3, 2]`. Lehmer will either
`panic` or produce incorrect results for other vectors.

### Benchmarks

Benchmarks can be run with `cargo bench`:

```
test benchmark_from_decimal     ... bench:         264 ns/iter (+/- 10)
test benchmark_from_permutation ... bench:          83 ns/iter (+/- 9)
test benchmark_max_value        ... bench:          13 ns/iter (+/- 1)
test benchmark_to_decimal       ... bench:          40 ns/iter (+/- 2)
test benchmark_to_permutation   ... bench:         142 ns/iter (+/- 9)
```

e.g. `Lehmer::from_permutation` runs at ~13 million iterations per second.

### Tests

Tests can be run with `cargo test`. Unit tests are in files next to their
modules and integration tests are in `tests/`.

### Contributions

Contributions are welcome. Please test/benchmark your changes and open a PR.
