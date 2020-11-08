#![allow(dead_code, non_snake_case)]

use crate::Bit::{O, S};

fn main(){}

// O -> 0, S -> 1
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bit{
    O,
    S
}

pub fn Not(a: Bit) -> Bit {
    Nand(a, a)
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

#[cfg(test)]
mod tests {
    use super::Bit::{O, S};
    use super::{Nand, Not};
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
}
