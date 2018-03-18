use super::*;

type Subject = BitString;

mod new {
    use super::*;

    #[test]
    fn it_is_initialized_to_zero() {
        let subject = Subject::new();
        assert_eq!(subject.b, 0);
    }
}

mod set {
    use super::*;

    #[test]
    fn it_sets_the_nth_bit() {
        let mut subject = Subject::new();

        subject.set(0);
        assert_eq!(1, subject.b);

        subject.set(2);
        assert_eq!(5, subject.b);

        subject.set(31);
        assert_eq!(2147483653, subject.b);
    }

    #[test]
    #[should_panic]
    fn it_panics_if_n_is_out_of_bounds() {
        let mut subject = Subject::new();
        subject.set(32);
    }
}

mod count_until {
    use super::*;

    #[test]
    fn it_counts_the_number_of_set_bits_until_the_nth_bit() {
        let mut subject = Subject::new();

        subject.set(1);
        subject.set(3);

        assert_eq!(subject.count_until(0), 0);
        assert_eq!(subject.count_until(1), 0);
        assert_eq!(subject.count_until(2), 1);
        assert_eq!(subject.count_until(3), 1);
        assert_eq!(subject.count_until(4), 2);
        assert_eq!(subject.count_until(32), 2);
    }

    #[test]
    #[should_panic]
    fn it_panics_if_n_is_out_of_bounds() {
        let subject = Subject::new();
        subject.count_until(33);
    }
}
