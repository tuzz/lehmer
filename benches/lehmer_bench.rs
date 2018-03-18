#[macro_use] extern crate bencher;
extern crate lehmer;

use bencher::Bencher;
use lehmer::Lehmer;

fn benchmark_from_permutation(bench: &mut Bencher) {
    bench.iter(|| {
        let vec = vec![7, 15, 0, 1, 2, 6, 3, 12, 5, 14, 8, 11, 9, 4, 13, 10];
        Lehmer::from_permutation(vec);
    })
}

fn benchmark_from_decimal(bench: &mut Bencher) {
    bench.iter(|| {
        Lehmer::from_decimal(10_374_227_790_679, 16);
    })
}

fn benchmark_to_permutation(bench: &mut Bencher) {
    bench.iter(|| {
        let code = vec![7, 14, 0, 0, 0, 3, 0, 6, 1, 6, 1, 3, 1, 0, 1, 0];
        Lehmer { code }.to_permutation();
    })
}

fn benchmark_to_decimal(bench: &mut Bencher) {
    bench.iter(|| {
        let code = vec![7, 14, 0, 0, 0, 3, 0, 6, 1, 6, 1, 3, 1, 0, 1, 0];
        Lehmer { code }.to_decimal();
    })
}

fn benchmark_max_value(bench: &mut Bencher) {
    bench.iter(|| {
        Lehmer::max_value(10)
    })
}

benchmark_group!(
    benches,
    benchmark_from_permutation,
    benchmark_from_decimal,
    benchmark_to_permutation,
    benchmark_to_decimal,
    benchmark_max_value,
);

benchmark_main!(benches);
