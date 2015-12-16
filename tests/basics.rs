extern crate editdistancewf as wf;

#[cfg(test)]
mod basic_distances {
    use wf;

    #[test]
    pub fn two_identical_sequences_have_a_distance_of_zero () {
        let left  = "left";
        let right = "right";

        assert_eq!(wf::distance(left.chars(), right.chars()), 0)
    }

    #[test]
    pub fn simple_differences () {
        assert_eq!(wf::distance("left".chars(), "lend".chars()), 2)
    }
}
