#![allow(dead_code, non_snake_case)]

use crate::Bit::{O, S};

fn main(){}

// O -> 0, S -> 1
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bit{
    O,
    S
}

pub fn Nand(a: Bit, b: Bit) -> Bit {
    match a {
        O => match b {
            O => S,
            S => S
        },
        S => match b {
            O => S,
            S => O
        }
    }
}

pub fn Not(a: Bit) -> Bit {
    Nand(a, a)
}

pub fn And(a: Bit, b: Bit) -> Bit {
    Nand(
        Nand(a, b),
        Nand(a, b)
    )
}

pub fn Or(a: Bit, b: Bit) -> Bit {
    Nand(
        Nand(a, a),
        Nand(b, b)
    )
}

pub fn Xor(a: Bit, b: Bit) -> Bit {
    Or(
        And(
            a,
            Not(b)
        ),
        And(
            Not(a),
            b
        )
    )
}

pub fn Mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    Or(
        And(
            a,
            Not(sel)
        ),
        And(
            b,
            sel
        )
    )
}

pub fn DMux(a: Bit, sel: Bit) -> (Bit, Bit) {
    (
        And(
            a,
            Not(sel)
        ),
        And(
            a,
            sel
        )
    )
}

#[cfg(test)]
mod tests {
    use super::Bit::{O, S};
    use super::{Nand, Not, And, Or, Xor, Mux, DMux};
    #[test]
    fn for_nand() {
        assert_eq!(Nand(O, O), S);
        assert_eq!(Nand(O, S), S);
        assert_eq!(Nand(S, O), S);
        assert_eq!(Nand(S, S), O);

    }

    #[test]
    fn for_not() {
        assert_eq!(Not(O), S);
        assert_eq!(Not(S), O);
    }

    #[test]
    fn for_and() {
        assert_eq!(And(O, O), O);
        assert_eq!(And(O, S), O);
        assert_eq!(And(S, O), O);
        assert_eq!(And(S, S), S);
    }

    #[test]
    fn for_or() {
        assert_eq!(Or(O, O), O);
        assert_eq!(Or(O, S), S);
        assert_eq!(Or(S, O), S);
        assert_eq!(Or(S, S), S);
    }

    #[test]
    fn for_xor() {
        assert_eq!(Xor(O, O), O);
        assert_eq!(Xor(O, S), S);
        assert_eq!(Xor(S, O), S);
        assert_eq!(Xor(S, S), O);
    }

    #[test]
    fn for_mux() {
        assert_eq!(Mux(O, O, O), O);
        assert_eq!(Mux(O, S, O), O);
        assert_eq!(Mux(S, O, O), S);
        assert_eq!(Mux(S, S, O), S);
        assert_eq!(Mux(O, O, S), O);
        assert_eq!(Mux(O, S, S), S);
        assert_eq!(Mux(S, O, S), O);
        assert_eq!(Mux(S, S, S), S);
    }

    #[test]
    fn for_dmux() {
        assert_eq!(DMux(O, O), (O, O));
        assert_eq!(DMux(O, S), (O, O));
        assert_eq!(DMux(S, O), (S, O));
        assert_eq!(DMux(S, S), (O, S));
    }
}
