mod logic;

use logic::*;
use logic::Bit::{O, I};

fn main() {
    println!("{}", And(I, O));
}