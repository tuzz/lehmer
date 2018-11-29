extern crate lehmer;

use lehmer::Lehmer;

#[test]
fn it_can_convert_between_permutations_lehmer_codes_and_decimals() {
    assert_example(&[0, 1], &[0, 0], 0);
    assert_example(&[1, 0], &[1, 0], 1);

    assert_example(&[0, 1, 2, 3, 4], &[0, 0, 0, 0, 0], 0);
    assert_example(&[0, 1, 2, 4, 3], &[0, 0, 0, 1, 0], 1);
    assert_example(&[1, 0, 4, 3, 2], &[1, 0, 2, 1, 0], 29);
    assert_example(&[3, 4, 1, 2, 0], &[3, 3, 1, 1, 0], 93);
    assert_example(&[4, 3, 2, 1, 0], &[4, 3, 2, 1, 0], 119);

    assert_example(&[0, 1, 2, 3, 4, 5, 6], &[0, 0, 0, 0, 0, 0, 0], 0);
    assert_example(&[2, 1, 5, 4, 6, 0, 3], &[2, 1, 3, 2, 2, 0, 0], 1648);
    assert_example(&[5, 0, 1, 3, 6, 2, 4], &[5, 0, 0, 1, 2, 0, 0], 3610);
    assert_example(&[4, 2, 6, 1, 0, 5, 3], &[4, 2, 4, 1, 0, 1, 0], 3223);
    assert_example(&[6, 5, 4, 3, 2, 1, 0], &[6, 5, 4, 3, 2, 1, 0], 5039);

    assert_example(
        &[7, 12, 14, 4, 3, 20, 5, 9, 6, 11, 0, 18, 10, 16, 1, 2, 8, 17, 15, 19, 13],
        &[7, 11, 12, 4, 3, 15, 3, 5, 3, 5, 0, 8, 3, 5, 0, 0, 0, 2, 1, 1, 0],
        <usize>::max_value(),
    );
}

fn assert_example(permutation: &[u8], code: &[u8], decimal: usize) {
    let mut lehmer: Lehmer;

    lehmer = Lehmer::from_permutation(permutation);
    assert_eq!(lehmer.code, code);
    assert_eq!(lehmer.to_decimal(), decimal);

    lehmer = Lehmer::from_decimal(decimal, code.len());
    assert_eq!(lehmer.code, code);
    assert_eq!(lehmer.to_permutation(), permutation);
}
