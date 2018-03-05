use super::*;

type Subject = Lehmer;

mod new {
    use super::*;

    #[test]
    fn it_is_initialized_from_the_vector() {
        let subject = Subject::new(vec![0, 1, 2]);
        assert_eq!(subject.vec, vec![0, 1, 2]);
    }
}

mod from_permutation {
    use super::*;

    fn result(vec: Vec<u64>) -> Vec<u64> {
        Subject::from_permutation(vec).vec
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

mod to_decimal {
    use super::*;

    fn result(vec: Vec<u64>) -> u64 {
        Subject::new(vec).to_decimal()
    }

    #[test]
    fn it_maps_a_number_in_factorial_base_to_its_decimal_representation() {
        assert_eq!(result(vec![0, 0, 0]), 0);
        assert_eq!(result(vec![0, 1, 0]), 1);
        assert_eq!(result(vec![1, 0, 0]), 2);
        assert_eq!(result(vec![1, 1, 0]), 3);
        assert_eq!(result(vec![2, 0, 0]), 4);
        assert_eq!(result(vec![2, 1, 0]), 5);
    }

    #[test]
    fn it_works_for_the_trivial_case() {
        assert_eq!(result(vec![0]), 0);
    }
}