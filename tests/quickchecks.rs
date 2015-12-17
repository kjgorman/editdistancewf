extern crate editdistancewf as wf;
extern crate quickcheck;
use quickcheck::quickcheck;

#[test]
fn at_least_size_difference_property() {
    fn at_least_size_difference(a: String, b: String) -> bool {
        let size_a = a.chars().count() as isize;
        let size_b = b.chars().count() as isize;
        let diff = (size_a - size_b).abs() as usize;

        wf::distance(a.chars(), b.chars()) >= diff
    }

    quickcheck(at_least_size_difference as fn(a: String, b: String) -> bool);
}

#[test]
fn at_most_length_of_longer_property() {
    fn at_most_size_of_longer(a: String, b: String) -> bool {
        let upper_bound = *[a.chars().count(),
                            b.chars().count()]
            .iter()
            .max()
            .unwrap() as usize;

        wf::distance(a.chars(), b.chars()) <= upper_bound
    }

    quickcheck(at_most_size_of_longer as fn(a: String, b: String) -> bool);
}

#[test]
fn zero_iff_a_equals_b_property() {
    fn zero_iff_a_equals_b(a: String, b: String) -> bool {
        let d = wf::distance(a.chars(), b.chars());

        if a == b {
            d == 0
        } else {
            d > 0
        }
    }

    quickcheck(zero_iff_a_equals_b as fn(a: String, b: String) -> bool);
}

#[test]
fn triangle_inequality_property() {
    fn triangle_inequality(a: String, b: String, c: String) -> bool {
        wf::distance(a.chars(), b.chars()) <=
            wf::distance(a.chars(), c.chars()) +
            wf::distance(b.chars(), c.chars())
    }

    quickcheck(triangle_inequality as fn(a: String, b: String, c: String) -> bool);
}
