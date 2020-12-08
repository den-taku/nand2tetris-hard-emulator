#![allow(unused_imports)]

mod logic;
mod arithmetic;
mod sequential;
mod architecture;

use logic::*;
use logic::bit::{O, I};
use architecture::*;
use arithmetic::*;
use sequential::*;
use sequential::ClockState::{Tick, Tock};

fn main() {
    let mut computer = Computer::new();
    computer.run();

    // let mut pc = PC::new();
    // let mut clock = Clock::new();
    // let mut memory = Memory::new();
    // // clock.next();
    // for _ in 0..80 {
    //     memory.keyboard_input(&clock);
    //     pc.input(&clock, Word::new([I; 16]), I, O, O);
    //     let out = pc.output(&clock);
    //     memory.input(&clock, Word::new([I; 16]), [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I], I);
    //     let memory_out = memory.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]);
    //     // println!("");
    //     // println!("times: {}, PC: {:?}", i, pc);
    //     if clock.state() == Tock {
    //         println!("");
    //         println!("{}", out);
    //         println!("memory");
    //         println!("{}", memory_out);
    //     }
    //     clock.next();
    // }

    // let mut clock = Clock::new();
    // loop {
    //     clock.next();
    //     println!("{:?}", clock.state());
    // }
}
