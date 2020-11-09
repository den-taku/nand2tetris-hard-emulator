mod logic;
mod arithmetic;

use logic::*;
use logic::Bit::{O, I};

fn main() {
    println!("{}", And(I, O));
    println!("{}", Word::new([O; 16]));
    println!("{}", Word::new([I; 16]));
}