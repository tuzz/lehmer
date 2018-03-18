use super::*;

type Subject = Lehmer;

mod from_permutation {
    use super::*;

    fn result(vec: Vec<u64>) -> Vec<u64> {
        Subject::from_permutation(vec).code
    }

    #[test]
    fn it_maps_a_permutation_to_its_lehmer_code() {
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

    #[test]
    fn it_works_for_a_complicated_case() {
        assert_eq!(result(vec![1, 0, 4, 3, 2]), vec![1, 0, 2, 1, 0]);
    }
}

mod from_decimal {
    use super::*;

    fn result(decimal: u64, n: usize) -> Vec<u64> {
        Subject::from_decimal(decimal, n).code
    }

    #[test]
    fn it_maps_a_decimal_number_to_its_lehmer_code() {
        assert_eq!(result(0, 3), vec![0, 0, 0]);
        assert_eq!(result(1, 3), vec![0, 1, 0]);
        assert_eq!(result(2, 3), vec![1, 0, 0]);
        assert_eq!(result(3, 3), vec![1, 1, 0]);
        assert_eq!(result(4, 3), vec![2, 0, 0]);
        assert_eq!(result(5, 3), vec![2, 1, 0]);
    }

    #[test]
    fn it_works_for_the_trivial_case() {
        assert_eq!(result(0, 1), vec![0]);
    }

    #[test]
    fn it_works_for_a_complicated_case() {
        assert_eq!(result(29, 5), vec![1, 0, 2, 1, 0]);
    }
}

mod to_permutation {
    use super::*;

    fn result(code: Vec<u64>) -> Vec<u64> {
        Subject { code }.to_permutation()
    }

    #[test]
    fn it_maps_a_lehmer_code_to_its_permutation() {
        assert_eq!(result(vec![0, 0, 0]), vec![0, 1, 2]);
        assert_eq!(result(vec![0, 1, 0]), vec![0, 2, 1]);
        assert_eq!(result(vec![1, 0, 0]), vec![1, 0, 2]);
        assert_eq!(result(vec![1, 1, 0]), vec![1, 2, 0]);
        assert_eq!(result(vec![2, 0, 0]), vec![2, 0, 1]);
        assert_eq!(result(vec![2, 1, 0]), vec![2, 1, 0]);
    }

    #[test]
    fn it_works_for_the_trivial_case() {
        assert_eq!(result(vec![0]), vec![0]);
    }

    #[test]
    fn it_works_for_a_complicated_case() {
        assert_eq!(result(vec![1, 0, 2, 1, 0]), vec![1, 0, 4, 3, 2]);
    }
}

mod to_decimal {
    use super::*;

    fn result(code: Vec<u64>) -> u64 {
        Subject { code }.to_decimal()
    }

    #[test]
    fn it_maps_a_lehmer_code_to_its_decimal_number() {
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

    #[test]
    fn it_works_for_a_complicated_case() {
        assert_eq!(result(vec![1, 0, 2, 1, 0]), 29);
    }
}

mod max_value {
    use super::*;

    #[test]
    fn it_returns_the_maximum_decimal_for_a_permutation_of_n_items() {
        assert_eq!(Subject::max_value(0), 0);
        assert_eq!(Subject::max_value(1), 0);
        assert_eq!(Subject::max_value(2), 1);
        assert_eq!(Subject::max_value(3), 5);
        assert_eq!(Subject::max_value(4), 23);
        assert_eq!(Subject::max_value(5), 119);

        let max = Subject::max_value(5);
        let code = Lehmer::from_decimal(max, 5);
        let permutation = code.to_permutation();

        assert_eq!(permutation, vec![4, 3, 2, 1, 0]);
    }
}
