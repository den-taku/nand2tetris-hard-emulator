#![allow(dead_code, non_snake_case)]

use crate::ClockState::{Tick, Tock};
use crate::logic::{bit, bit::O, *};

// tick: input, update internal state
// tock: output 
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockState {
    Tick, // 0 (clock's state starts with this)
    Tock  // 1 
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Clock{
    state: ClockState, // ture: 
}

impl Clock {
    pub fn state(&self) -> ClockState {
        self.state
    }
    pub fn next(&mut self) {
        self.state = match self.state {
            Tick => Tock,
            Tock => Tick
        };
    }
    pub fn new() -> Self {
        Clock{ state: Tick }
    }
}

// must use input and output both
// when use, save input value and output one separately in variables
// use input first
// primitive sequential gate
#[derive(Debug, Copy, Clone)]
pub struct DFF {
    state_past: bit, // use when tick
    state_new: bit // use when tock
}

impl DFF {
    pub fn new() -> Self {
        DFF { 
            state_past: O,
            state_new: O
        }
    }

    pub fn input(&mut self, a: bit, clock: &Clock) {
        if clock.state() == Tick {
            self.state_past = self.state_new;
            self.state_new = a
        }
    }

    pub fn output(self, clock: &Clock) -> bit {
        if clock.state() == Tick {
            self.state_past
        } else {
            self.state_new
        }
    }
}

// must use input and output both
// when use, save input value and output one separately in variables
// use input first
#[derive(Debug, Copy, Clone)]
pub struct Bit {
    dff: DFF
}

impl Bit {
    pub fn new() -> Self {
        Bit { dff: DFF::new() }
    }

    pub fn input(&mut self, clock: &Clock, input: bit, load: bit) {
        let clock_tmp = match clock.state() {
            Tick => {
                let mut c = Clock::new();
                c.next();
                c
            },
            Tock => Clock::new()
        };
        self.dff.input(
            Mux(
                self.output(&clock_tmp),
                input,
                load
            ),
            &clock
        )
    }

    pub fn output(&self, clock: &Clock) -> bit {
        self.dff.output(&clock)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Register {
    bits: [Bit; 16]
} 

impl Register {
    pub fn new() -> Self {
        Register { bits: [Bit::new(); 16] }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, load: bit) {
        self.bits[0].input(clock, input[0], load);
        self.bits[1].input(clock, input[1], load);
        self.bits[2].input(clock, input[2], load);
        self.bits[3].input(clock, input[3], load);
        self.bits[4].input(clock, input[4], load);
        self.bits[5].input(clock, input[5], load);
        self.bits[6].input(clock, input[6], load);
        self.bits[7].input(clock, input[7], load);
        self.bits[8].input(clock, input[8], load);
        self.bits[9].input(clock, input[9], load);
        self.bits[10].input(clock, input[10], load);
        self.bits[11].input(clock, input[11], load);
        self.bits[12].input(clock, input[12], load);
        self.bits[13].input(clock, input[13], load);
        self.bits[14].input(clock, input[14], load);
        self.bits[15].input(clock, input[15], load);
    }

    pub fn output(&self, clock: &Clock) -> Word {
        Word::new([
            self.bits[0].output(clock),
            self.bits[1].output(clock),
            self.bits[2].output(clock),
            self.bits[3].output(clock),
            self.bits[4].output(clock),
            self.bits[5].output(clock),
            self.bits[6].output(clock),
            self.bits[7].output(clock),
            self.bits[8].output(clock),
            self.bits[9].output(clock),
            self.bits[10].output(clock),
            self.bits[11].output(clock),
            self.bits[12].output(clock),
            self.bits[13].output(clock),
            self.bits[14].output(clock),
            self.bits[15].output(clock),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ClockState::{Tick, Tock};
    use crate::logic::bit::{I, O};

    #[test]
    fn for_clock_new() {
        let mut clock = Clock::new();
        assert_eq!(clock.state(), Tick);
        clock.next();
        assert_eq!(clock.state(), Tock);
        clock.next();
        assert_eq!(clock.state(), Tick);
    }

    #[test]
    fn for_dff() {
        // initialize as past: O, new: O
        let mut dff = DFF::new();
        // initialize as state: Tick
        let mut clock = Clock::new();

        // input as past: O, new: I
        dff.input(I, &clock);
        // output past (=O)
        assert_eq!(dff.output(&clock), O);
        // Tock
        clock.next();

        // nothing happen
        dff.input(O, &clock);
        // output new (=I)
        assert_eq!(dff.output(&clock), I);
        // Tick
        clock.next();
        
        // input as past: I, new: O
        dff.input(O, &clock);
        // output past (=I)
        assert_eq!(dff.output(&clock), I);
        // Tock
        clock.next();

        // nothing happen
        dff.input(I, &clock);
        // output new (=O)
        assert_eq!(dff.output(&clock), O);
    }

    #[test]
    fn for_bit() {
        // initialize as past: O, new: O
        let mut bit = Bit::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        // input as past: O, new: I
        bit.input(&clock, I, I);
        // output past
        assert_eq!(bit.output(&clock), O);

        // Tock
        clock.next();

        // nothing happened
        bit.input(&clock, O, O);
        // output new
        assert_eq!(bit.output(&clock), I);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        bit.input(&clock, O, O);
        // output past
        assert_eq!(bit.output(&clock), I);

        // Tock
        clock.next();

        // nothing happened
        bit.input(&clock, O, I);
        // output new
        assert_eq!(bit.output(&clock), I);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        bit.input(&clock, O, I);
        // output past
        assert_eq!(bit.output(&clock), I);

        // Tock
        clock.next();

        // nothing happened
        bit.input(&clock, I, O);
        // output new
        assert_eq!(bit.output(&clock), O);


    }

    #[test]
    fn for_register() {
        // initialize as past: O, new: O
        let mut register = Register::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I; 16]);
        let word_o = Word::new([O; 16]);

        // input as past: O, new: I
        register.input(&clock, word_i, I);
        // output past
        assert_eq!(register.output(&clock), word_o);

        // Tock
        clock.next();

        // nothing happened
        register.input(&clock, word_o, O);
        // output new
        assert_eq!(register.output(&clock), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        register.input(&clock, word_o, O);
        // output past
        assert_eq!(register.output(&clock), word_i);

        // Tock
        clock.next();

        // nothing happened
        register.input(&clock, word_o, I);
        // output new
        assert_eq!(register.output(&clock), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        register.input(&clock, word_o, I);
        // output past
        assert_eq!(register.output(&clock), word_i);

        // Tock
        clock.next();

        // nothing happened
        register.input(&clock, word_i, O);
        // output new
        assert_eq!(register.output(&clock), word_o);


    }
}