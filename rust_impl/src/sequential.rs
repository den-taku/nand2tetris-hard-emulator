#![allow(dead_code, non_snake_case)]

use crate::ClockState::{Tick, Tock};
use crate::logic::{bit, bit::O, Mux};

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
}