#![allow(dead_code, non_snake_case)]

use crate::Bit::{O, I};
use std::fmt;
use std::fmt::{Display, Formatter};

fn main(){
    println!("{}", Not(Mux(I, O, I)));
    println!("{:?}", [I, O, I, O, I, I, O]);
}

// O -> 0, I -> 1
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bit{
    O,
    I
}

impl Display for Bit {
    fn fmt(&self, dest: &mut Formatter) -> fmt::Result {
        let buf = match self {
            I => "I".to_string(),
            O => "O".to_string()
        };
        write!(dest, "{}", buf)
    }
}

pub fn Nand(a: Bit, b: Bit) -> Bit {
    match a {
        O => match b {
            O => I,
            I => I
        },
        I => match b {
            O => I,
            I => O
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
    use super::Bit::{O, I};
    use super::{Nand, Not, And, Or, Xor, Mux, DMux};
    #[test]
    fn for_nand() {
        assert_eq!(Nand(O, O), I);
        assert_eq!(Nand(O, I), I);
        assert_eq!(Nand(I, O), I);
        assert_eq!(Nand(I, I), O);

    }

    #[test]
    fn for_not() {
        assert_eq!(Not(O), I);
        assert_eq!(Not(I), O);
    }

    #[test]
    fn for_and() {
        assert_eq!(And(O, O), O);
        assert_eq!(And(O, I), O);
        assert_eq!(And(I, O), O);
        assert_eq!(And(I, I), I);
    }

    #[test]
    fn for_or() {
        assert_eq!(Or(O, O), O);
        assert_eq!(Or(O, I), I);
        assert_eq!(Or(I, O), I);
        assert_eq!(Or(I, I), I);
    }

    #[test]
    fn for_xor() {
        assert_eq!(Xor(O, O), O);
        assert_eq!(Xor(O, I), I);
        assert_eq!(Xor(I, O), I);
        assert_eq!(Xor(I, I), O);
    }

    #[test]
    fn for_mux() {
        assert_eq!(Mux(O, O, O), O);
        assert_eq!(Mux(O, I, O), O);
        assert_eq!(Mux(I, O, O), I);
        assert_eq!(Mux(I, I, O), I);
        assert_eq!(Mux(O, O, I), O);
        assert_eq!(Mux(O, I, I), I);
        assert_eq!(Mux(I, O, I), O);
        assert_eq!(Mux(I, I, I), I);
    }

    #[test]
    fn for_dmux() {
        assert_eq!(DMux(O, O), (O, O));
        assert_eq!(DMux(O, I), (O, O));
        assert_eq!(DMux(I, O), (I, O));
        assert_eq!(DMux(I, I), (O, I));
    }
}
