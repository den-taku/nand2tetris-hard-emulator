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
    let mut rom = ROM32K::new();
    rom.load("test.txt");
    
    // initialize as past: O, new: O
    let mut ram = Memory::new();
    // initialize state as Tick
    let mut clock = Clock::new();

    let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
    let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
    let word_0 = Word::new([O; 16]);

    // input as past: word_0, new: word_i in registers
    ram.input(&clock, word_i, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
    // output past in register
    assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_0);

    // Tock
    clock.next();

    // nothing happened
    ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O], O);
    // output new
    assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

    // Tick
    clock.next();

    // initialize as past: I, new: I
    ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O], O);
    // output past
    assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

    // Tock
    clock.next();

    // nothing happened
    ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
    // output new
    assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

    // Tick
    clock.next();

    // initialize as past: I, new: O
    ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
    // output past
    assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

    // Tock
    clock.next();

    // nothing happened
    ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
    // output new
    assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_o);
    
    clock.next();

    ram.input(&clock, word_o, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], I);
    assert_eq!(ram.output(&clock, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O]), word_0);

    clock.next();

    ram.input(&clock, word_i, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], O);
    assert_eq!(ram.output(&clock, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O]), word_o);

    clock.next();

    ram.input(&clock, word_i, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], O);
    assert_eq!(ram.output(&clock, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O]), word_o);

    clock.next();

    ram.input(&clock, word_i, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], I);
    assert_eq!(ram.output(&clock, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O]), word_o);

    clock.next();

    ram.input(&clock, word_i, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], I);
    assert_eq!(ram.output(&clock, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O]), word_o);

    clock.next();

    ram.input(&clock, word_i, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], I);
    assert_eq!(ram.output(&clock, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O]), word_i);

    clock.next();

    ram.input(&clock, word_i, [I, O, I, O, O, I, O, O, I, O, O, I, O, O, O], I);
    assert_eq!(ram.output(&clock, [I, I, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_0);

    clock.next();
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
    let mut memory = Memory::new();
    // clock.next();
    for _ in 0..80 {
        memory.keyboard_input(&clock);
        pc.input(&clock, Word::new([I; 16]), I, O, O);
        let out = pc.output(&clock);
        memory.input(&clock, Word::new([I; 16]), [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I], I);
        let memory_out = memory.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]);
        // println!("");
        // println!("times: {}, PC: {:?}", i, pc);
        if clock.state() == Tock {
            println!("");
            println!("{}", out);
            println!("memory");
            println!("{}", memory_out);
        }
        clock.next();
    }

    // let mut clock = Clock::new();
    // loop {
    //     clock.next();
    //     println!("{:?}", clock.state());
    // }
}