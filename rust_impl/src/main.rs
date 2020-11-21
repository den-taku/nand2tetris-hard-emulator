#![allow(unused_imports)]

mod logic;
mod arithmetic;
mod sequential;
mod architecture;

use logic::*;
use logic::bit::{O, I};
use arithmetic::*;
use sequential::*;
use sequential::ClockState::{Tick, Tock};

fn main() {
    // println!("{}", And(I, O));
    // println!("{}", Word::new([O; 16]));
    // println!("{}", Word::new([I; 16]));
    // println!("{}", Inc16(Word::new([I; 16])));
    // println!("{:?}", 
    //     ALU(
    //         Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
    //         Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
    //         O, O, I, I, O, O
    //     ));
    // let mut ram8 = RAM8::new();
    // let mut clock = Clock::new();
    // let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
    // let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
    // let word_0 = Word::new([O; 16]);
    // input as past: word_0, new: word_i in registers
    // ram8.input(&clock, word_i, [O, I, O], I);
    // output past in register
    // assert_eq!(ram8.output(&clock, [O, I, O]), word_0);

    // println!("");
    // println!("{:?}", ram8);
    // println!("");

    // Tock
    // clock.next();

    // nothing happened
    // ram8.input(&clock, word_o, [O, I, O], O);
    // output new
    // assert_eq!(ram8.output(&clock, [O, I, O]), word_i);

    // println!("{:?}", ram8);

    // Tick
    // clock.next();

    let mut pc = PC::new();
    let mut clock = Clock::new();
    // clock.next();
    for _ in 0..80 {
        pc.input(&clock, Word::new([I; 16]), I, O, O);
        let out = pc.output(&clock);
        // println!("");
        // println!("times: {}, PC: {:?}", i, pc);
        if clock.state() == Tock {
            println!("");
            println!("{}", out);
        }
        clock.next();
    }
    
    // let mut clock = Clock::new();
    // loop {
    //     clock.next();
    //     println!("{:?}", clock.state());
    // }
}