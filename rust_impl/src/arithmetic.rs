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
pub fn FullAdder(a: Bit, b: Bit, c: Bit) -> [Bit; 2] {
    unimplemented!()
}


#[cfg(test)]
mod tests{
    // use crate::logic::*;
    use crate::logic::Bit::{I, O};
    use super::{HalfAdder};

    #[test]
    fn for_halfadder() {
        assert_eq!(HalfAdder(O, O), [O, O]);
        assert_eq!(HalfAdder(O, I), [O, I]);
        assert_eq!(HalfAdder(I, O), [O, I]);
        assert_eq!(HalfAdder(I, I), [I, O]);
    }
}