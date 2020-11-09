#![allow(dead_code, non_snake_case)]

use crate::logic::*;
// use crate::logic::Bit::{I, O};

// a, b -> curry, sum
pub fn HalfAdder(a: Bit, b: Bit) -> [Bit; 2] {
    [
        And(
            a,
            b
        ),
        Xor(
            a,
            b
        )
    ]
}

// a, b, curry -> curry, sum
// c = a * b + b * curry + curry * a
pub fn FullAdder(a: Bit, b: Bit, c: Bit) -> [Bit; 2] {
    let half_adder1 = HalfAdder(
        a,
        b
    );
    let half_adder2 = HalfAdder(
        c,
        half_adder1[1]
    );
    [
        Or(
            half_adder1[0],
            half_adder2[0]
        ),
        half_adder2[1]
    ]
}


#[cfg(test)]
mod tests{
    // use crate::logic::*;
    use crate::logic::Bit::{I, O};
    use super::{HalfAdder, FullAdder};

    #[test]
    fn for_halfadder() {
        assert_eq!(HalfAdder(O, O), [O, O]);
        assert_eq!(HalfAdder(O, I), [O, I]);
        assert_eq!(HalfAdder(I, O), [O, I]);
        assert_eq!(HalfAdder(I, I), [I, O]);
    }

    #[test]
    fn for_fulladder() {
        assert_eq!(FullAdder(O, O, O), [O, O]);
        assert_eq!(FullAdder(O, O, I), [O, I]);
        assert_eq!(FullAdder(O, I, O), [O, I]);
        assert_eq!(FullAdder(O, I, I), [I, O]);
        assert_eq!(FullAdder(I, O, O), [O, I]);
        assert_eq!(FullAdder(I, O, I), [I, O]);
        assert_eq!(FullAdder(I, I, O), [I, O]);
        assert_eq!(FullAdder(I, I, I), [I, I]);
    }
}