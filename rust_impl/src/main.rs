#![allow(dead_code, non_snake_case)]

use crate::Bit::{O, I};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

fn main(){
    println!("{}", Not(Mux(I, O, I)));
    println!("{:?}", [I, O, I, O, I, I, O]);
    // let _word = Word([I; 16]);
    // println!("kkk");
    println!("{}", Word([I; 16]));
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Word ([Bit; 16]);

impl Word {
    pub fn new(a: [Bit; 16]) -> Self {
        Word(a)
    }
    pub fn to_slice(&self) -> [Bit; 16] {
        [
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            self[5],
            self[6],
            self[7],
            self[8],
            self[9],
            self[10],
            self[11],
            self[12],
            self[13],
            self[14],
            self[15],
        ]
    }
}

impl Display for Word {
    fn fmt(&self, dest: &mut Formatter) -> fmt::Result {
        let mut buf = "[".to_string();
        for i in 0..16 {
            buf = format!("{} {},", buf, self[i]);
        }
        buf = format!("{} ]", buf);
        write!(dest, "{}", buf)
    }
}

impl Index<usize> for Word {
    type Output = Bit;
    fn index(&self, index: usize) -> &Self::Output {
        if 15 < index {
            panic!(format!("index fail: {} is out of range.", index));
        }
        &self.0[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if 15 < index {
            panic!(format!("index_mut fail: {} is out of range.", index));
        }
        self.0.index_mut(index)
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

pub fn DMux(a: Bit, sel: Bit) -> [Bit; 2] {
    [
        And(
            a,
            Not(sel)
        ),
        And(
            a,
            sel
        )
    ]
}

pub fn Not16(a: Word) -> Word {
    Word::new([
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
    ])
}

pub fn And16(a: Word, b: Word) -> Word {
    Word::new([
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
    ])
} 

pub fn Or16(a: Word, b: Word) -> Word {
    Word::new([
        Or(a[0], b[0]),
        Or(a[1], b[1]),
        Or(a[2], b[2]),
        Or(a[3], b[3]),
        Or(a[4], b[4]),
        Or(a[5], b[5]),
        Or(a[6], b[6]),
        Or(a[7], b[7]),
        Or(a[8], b[8]),
        Or(a[9], b[9]),
        Or(a[10], b[10]),
        Or(a[11], b[11]),
        Or(a[12], b[12]),
        Or(a[13], b[13]),
        Or(a[14], b[14]),
        Or(a[15], b[15]),
    ])
}

pub fn Mux16(a: Word, b: Word, sel: Bit) -> Word {
    Word::new([
        Mux(a[0], b[0], sel),
        Mux(a[1], b[1], sel),
        Mux(a[2], b[2], sel),
        Mux(a[3], b[3], sel),
        Mux(a[4], b[4], sel),
        Mux(a[5], b[5], sel),
        Mux(a[6], b[6], sel),
        Mux(a[7], b[7], sel),
        Mux(a[8], b[8], sel),
        Mux(a[9], b[9], sel),
        Mux(a[10], b[10], sel),
        Mux(a[11], b[11], sel),
        Mux(a[12], b[12], sel),
        Mux(a[13], b[13], sel),
        Mux(a[14], b[14], sel),
        Mux(a[15], b[15], sel),
    ])
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

pub fn Mux4Way16(a: Word, b: Word, c: Word, d: Word, sel: [Bit; 2]) -> Word {
    let Mux2 = |a: Bit, b: Bit, c: Bit, d: Bit, s0: Bit, s1: Bit| -> Bit {
        Mux(
              Mux(
                    a,
                    b,
                    s1
                ),
                Mux(
                    c,
                    d,
                    s1
                ),
                s0
            )
    };
    Word::new([
        Mux2(a[0], b[0], c[0], d[0], sel[0], sel[1]),
        Mux2(a[1], b[1], c[1], d[1], sel[0], sel[1]),
        Mux2(a[2], b[2], c[2], d[2], sel[0], sel[1]),
        Mux2(a[3], b[3], c[3], d[3], sel[0], sel[1]),
        Mux2(a[4], b[4], c[4], d[4], sel[0], sel[1]),
        Mux2(a[5], b[5], c[5], d[5], sel[0], sel[1]),
        Mux2(a[6], b[6], c[6], d[6], sel[0], sel[1]),
        Mux2(a[7], b[7], c[7], d[7], sel[0], sel[1]),
        Mux2(a[8], b[8], c[8], d[8], sel[0], sel[1]),
        Mux2(a[9], b[9], c[9], d[9], sel[0], sel[1]),
        Mux2(a[10], b[10], c[10], d[10], sel[0], sel[1]),
        Mux2(a[11], b[11], c[11], d[11], sel[0], sel[1]),
        Mux2(a[12], b[12], c[12], d[12], sel[0], sel[1]),
        Mux2(a[13], b[13], c[13], d[13], sel[0], sel[1]),
        Mux2(a[14], b[14], c[14], d[14], sel[0], sel[1]),
        Mux2(a[15], b[15], c[15], d[15], sel[0], sel[1]),
    ])
}

pub fn Mux8Way16(a: Word, b: Word, c: Word, d: Word, e: Word, f: Word, g: Word, h: Word, sel: [Bit; 3]) -> Word {
    let Mux3 = |a: Bit, b: Bit, c: Bit, d: Bit, e: Bit, f: Bit, g: Bit, h: Bit, s0: Bit, s1: Bit, s2: Bit| -> Bit {
        Mux(
            Mux(
                Mux(
                    a,
                    b,
                    s2
                ),
                Mux(
                    c,
                    d,
                    s2
                ),
                s1
            ),
            Mux(
                Mux(
                    e,
                    f,
                    s2
                ),
                Mux(
                    g,
                    h,
                    s2
                ),
                s1
            ),
            s0
        )
    };
    Word::new(
        [
            Mux3(a[0], b[0], c[0], d[0], e[0], f[0], g[0], h[0], sel[0], sel[1], sel[2]),
            Mux3(a[1], b[1], c[1], d[1], e[1], f[1], g[1], h[1], sel[0], sel[1], sel[2]),
            Mux3(a[2], b[2], c[2], d[2], e[2], f[2], g[2], h[2], sel[0], sel[1], sel[2]),
            Mux3(a[3], b[3], c[3], d[3], e[3], f[3], g[3], h[3], sel[0], sel[1], sel[2]),
            Mux3(a[4], b[4], c[4], d[4], e[4], f[4], g[4], h[4], sel[0], sel[1], sel[2]),
            Mux3(a[5], b[5], c[5], d[5], e[5], f[5], g[5], h[5], sel[0], sel[1], sel[2]),
            Mux3(a[6], b[6], c[6], d[6], e[6], f[6], g[6], h[6], sel[0], sel[1], sel[2]),
            Mux3(a[7], b[7], c[7], d[7], e[7], f[7], g[7], h[7], sel[0], sel[1], sel[2]),
            Mux3(a[8], b[8], c[8], d[8], e[8], f[8], g[8], h[8], sel[0], sel[1], sel[2]),
            Mux3(a[9], b[9], c[9], d[9], e[9], f[9], g[9], h[9], sel[0], sel[1], sel[2]),
            Mux3(a[10], b[10], c[10], d[10], e[10], f[10], g[10], h[10], sel[0], sel[1], sel[2]),
            Mux3(a[11], b[11], c[11], d[11], e[11], f[11], g[11], h[11], sel[0], sel[1], sel[2]),
            Mux3(a[12], b[12], c[12], d[12], e[12], f[12], g[12], h[12], sel[0], sel[1], sel[2]),
            Mux3(a[13], b[13], c[13], d[13], e[13], f[13], g[13], h[13], sel[0], sel[1], sel[2]),
            Mux3(a[14], b[14], c[14], d[14], e[14], f[14], g[14], h[14], sel[0], sel[1], sel[2]),
            Mux3(a[15], b[15], c[15], d[15], e[15], f[15], g[15], h[15], sel[0], sel[1], sel[2]),
        ]
    )
}

pub fn DMux4Way(a: Bit, sel: [Bit; 2]) -> [Bit; 4] {
    [
        And(
            And(
                a,
                Not(sel[0])
            ),
            Not(sel[1])
        ),
        And(
            And(
                a,
                Not(sel[0])
            ),
            sel[1]
        ),
        And(
            And(
                a,
                sel[0]
            ),
            Not(sel[1])
        ),
        And(
            And(
                a,
                sel[0]
            ),
            sel[1]
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::Bit::{O, I};
    use super::Word;
    use super::{Nand, Not, And, Or, Xor, Mux, DMux, Not16, And16, Or16, Mux16,
                Or8Way, Mux4Way16, Mux8Way16, DMux4Way};
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
        assert_eq!(DMux(O, O), [O, O]);
        assert_eq!(DMux(O, I), [O, O]);
        assert_eq!(DMux(I, O), [I, O]);
        assert_eq!(DMux(I, I), [O, I]);
    }

    #[test]
    fn for_dislay() {
        assert_eq!(format!("{}", I), "I".to_string());
        assert_eq!(format!("{}", O), "O".to_string());
    }

    #[test]
    fn for_not16() {
        assert_eq!(
            Not16(Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            Not16(Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            Not16(Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            Not16(Word([O, O, O, O, O, O, O, O, I, I, I, I, I, I, I, I])),
            Word([I, I, I, I, I, I, I, I, O, O, O, O, O, O, O, O])
        );
    }

    #[test]
    fn for_and16() {
        assert_eq!(
            And16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            And16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            And16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            And16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            And16(
                Word([O, O, I, I, O, I, I, O, I, O, O, I, I, O, I, I]),
                Word([O, I, O, I, I, O, I, O, I, I, I, I, O, I, O, I]),
            ),
            Word([O, O, O, I, O, O, I, O, I, O, O, I, O, O, O, I])
        );
    }

    #[test]
    fn for_or16() {
        assert_eq!(
            Or16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            Or16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            Or16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            Or16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            Or16(
                Word([O, O, I, I, O, I, I, O, I, O, O, I, I, O, I, I]),
                Word([O, I, O, I, I, O, I, O, I, I, I, I, O, I, O, I]),
            ),
            Word([O, I, I, I, I, I, I, O, I, I, I, I, I, I, I, I])
        );
    }

    #[test]
    fn for_mux16() {
        assert_eq!(
            Mux16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                O
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            Mux16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                I
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            Mux16(
                Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                O
            ),
            Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            Mux16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O]),
                I
            ),
            Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O])
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

    #[test]
    fn for_mux4way16() {
        assert_eq!(
            Mux4Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]), 
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]), 
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]), 
                [O, O]
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            Mux4Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]), 
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]), 
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]), 
                [O, I]
            ),
            Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I] )
        );
        assert_eq!(
            Mux4Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]), 
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]), 
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]), 
                [I, O]
            ),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            Mux4Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]), 
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]), 
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]), 
                [I, I]
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
    }

    #[test]
    #[should_panic(expected = "index fail: 16 is out of range.")]
    fn for_index() {
        let _ = Word([I; 16])[16];
    }

    #[test]
    #[should_panic(expected = "index_mut fail: 16 is out of range.")]
    fn for_index_mut() {
        let mut dummy = Word([I; 16]);
        dummy[16] = O;
    }

    #[test]
    fn for_mux8way16() {
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, O, O]
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]) 
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, O, I]
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]) 
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, I, O]
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]) 
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, I, I]
            ),
            Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O])
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, O, O]
            ),
            Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O])
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, O, I]
            ),
            Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]) 
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, I, O]
            ),
            Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]) 
        );
        assert_eq!(
            Mux8Way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, I, I]
            ),
            Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O])
        );
    }

    #[test]
    fn for_dmux4way() {
        assert_eq!(DMux4Way(O, [O, O]), [O, O, O, O]);
        assert_eq!(DMux4Way(O, [O, I]), [O, O, O, O]);
        assert_eq!(DMux4Way(O, [I, O]), [O, O, O, O]);
        assert_eq!(DMux4Way(O, [I, I]), [O, O, O, O]);
        assert_eq!(DMux4Way(I, [O, O]), [I, O, O, O]);
        assert_eq!(DMux4Way(I, [O, I]), [O, I, O, O]);
        assert_eq!(DMux4Way(I, [I, O]), [O, O, I, O]);
        assert_eq!(DMux4Way(I, [I, I]), [O, O, O, I]);
    }
}
