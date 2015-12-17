extern crate editdistancewf as wf;

#[cfg(test)]
mod basic_distances {
    use wf;

    #[test]
    pub fn two_identical_sequences_have_a_distance_of_zero () {
        let left  = "left";
        let right = "left";

        assert_eq!(wf::distance(left.chars(), right.chars()), 0)
    }

    #[test]
    pub fn simple_insert () {
        assert_eq!(wf::distance("left".chars(), "lefty".chars()), 1)
    }

    #[test]
    pub fn simple_delete () {
        assert_eq!(wf::distance("left".chars(), "lef".chars()), 1)
    }

    #[test]
    pub fn simple_modification () {
        assert_eq!(wf::distance("left".chars(), "lefy".chars()), 1)
    }

    #[test]
    pub fn empty_comparisons () {
        assert_eq!(wf::distance("".chars(), "foo".chars()), 3);
        assert_eq!(wf::distance("foo".chars(), "".chars()), 3);
    }

    #[test]
    pub fn non_string_comparisons () {
        assert_eq!(wf::distance([1,2,3].iter(), [1,2,3].iter()), 0);
        assert_eq!(wf::distance([1,2].iter(), [1,2,3].iter()), 1);
    }
}
