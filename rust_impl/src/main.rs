mod logic;
mod arithmetic;

use logic::*;
use logic::Bit::{O, I};
use arithmetic::*;

fn main() {
    println!("{}", And(I, O));
    println!("{}", Word::new([O; 16]));
    println!("{}", Word::new([I; 16]));
    println!("{}", Inc16(Word::new([I; 16])));
}