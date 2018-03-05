use super::*;

mod from_permutation {
    use super::*;

    fn result(vec: Vec<u64>) -> Vec<u64> {
        Factorial::from_permutation(vec).digits
    }

    #[test]
    fn it_maps_a_permutation_to_its_representation_in_factorial_base() {
        assert_eq!(result(vec![0, 1, 2]), vec![0, 0, 0]);
        assert_eq!(result(vec![0, 2, 1]), vec![0, 1, 0]);
        assert_eq!(result(vec![1, 0, 2]), vec![1, 0, 0]);
        assert_eq!(result(vec![1, 2, 0]), vec![1, 1, 0]);
        assert_eq!(result(vec![2, 0, 1]), vec![2, 0, 0]);
        assert_eq!(result(vec![2, 1, 0]), vec![2, 1, 0]);
    }

    #[test]
    fn it_works_for_the_trivial_case() {
        assert_eq!(result(vec![0]), vec![0]);
    }
}
