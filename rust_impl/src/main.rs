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

pub fn Not16(a: [Bit; 16]) -> [Bit; 16] {
    [
        Not(a[0]),
        Not(a[1]),
        Not(a[2]),
        Not(a[3]),
        Not(a[4]),
        Not(a[5]),
        Not(a[6]),
        Not(a[7]),
        Not(a[8]),
        Not(a[9]),
        Not(a[10]),
        Not(a[11]),
        Not(a[12]),
        Not(a[13]),
        Not(a[14]),
        Not(a[15]),
    ]
}

pub fn And16(a: [Bit; 16], b: [Bit; 16]) -> [Bit; 16] {
    [
        And(a[0], b[0]),
        And(a[1], b[1]),
        And(a[2], b[2]),
        And(a[3], b[3]),
        And(a[4], b[4]),
        And(a[5], b[5]),
        And(a[6], b[6]),
        And(a[7], b[7]),
        And(a[8], b[8]),
        And(a[9], b[9]),
        And(a[10], b[10]),
        And(a[11], b[11]),
        And(a[12], b[12]),
        And(a[13], b[13]),
        And(a[14], b[14]),
        And(a[15], b[15]),
    ]
} 

pub fn Or8Way(a: [Bit; 8]) -> Bit {
    Or(
        Or(
            Or(
                a[0],
                a[1]
            ),
            Or(
                a[2],
                a[3]
            ),
        ),
        Or(
            Or(
                a[4],
                a[5]
            ),
            Or(
                a[6],
                a[7]
            )
        )
    )
}

pub fn Mux4Way16(a: [Bit; 16], b: [Bit; 16], c: [Bit; 16], d: [Bit; 16], sel: [Bit; 2]) -> [Bit; 16] {

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::Bit::{O, I};
    use super::{Nand, Not, And, Or, Xor, Mux, DMux, Not16, And16, Or8Way};
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

    #[test]
    fn for_dislay() {
        assert_eq!(format!("{}", I), "I".to_string());
        assert_eq!(format!("{}", O), "O".to_string());
    }

    #[test]
    fn for_not16() {
        assert_eq!(
            Not16([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]
        );
        assert_eq!(
            Not16([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]
        );
        assert_eq!(
            Not16([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
            [I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]
        );
        assert_eq!(
            Not16([O, O, O, O, O, O, O, O, I, I, I, I, I, I, I, I]),
            [I, I, I, I, I, I, I, I, O, O, O, O, O, O, O, O]
        );
    }

    #[test]
    fn for_and16() {
        assert_eq!(
            And16(
                [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O],
                [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O],
            ),
            [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]
        );
        assert_eq!(
            And16(
                [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I],
                [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O],
            ),
            [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]
        );
        assert_eq!(
            And16(
                [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O],
                [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I],
            ),
            [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]
        );
        assert_eq!(
            And16(
                [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I],
                [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I],
            ),
            [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]
        );
        assert_eq!(
            And16(
                [O, O, I, I, O, I, I, O, I, O, O, I, I, O, I, I],
                [O, I, O, I, I, O, I, O, I, I, I, I, O, I, O, I],
            ),
            [O, O, O, I, O, O, I, O, I, O, O, I, O, O, O, I]
        );
    }

    #[test]
    fn for_or8way() {
        assert_eq!(Or8Way([O, O, O, O, O, O, O, O],), O);
        assert_eq!(Or8Way([I, O, O, O, O, O, O, O],), I);
        assert_eq!(Or8Way([O, I, I, O, O, O, O, O],), I);
        assert_eq!(Or8Way([O, O, O, I, I, I, O, O],), I);
        assert_eq!(Or8Way([I, O, I, O, O, O, I, I],), I);
        assert_eq!(Or8Way([I, O, I, O, I, O, I, I],), I);
        assert_eq!(Or8Way([I, I, I, I, O, I, I, O],), I);
        assert_eq!(Or8Way([I, I, O, I, I, I, I, I],), I);
        assert_eq!(Or8Way([I, I, I, I, I, I, I, I],), I);
    }
}
