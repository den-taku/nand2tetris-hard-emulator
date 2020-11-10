#![allow(dead_code, non_snake_case)]

use crate::ClockState::{Tick, Tock};
use crate::logic::{Bit, Bit::O};

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
    state_past: Bit, // use when tick
    state_new: Bit // use when tock
}

impl DFF {
    pub fn new() -> DFF {
        DFF { 
            state_past: O,
            state_new: O
        }
    }

    pub fn input(&mut self, a: Bit, clock: &Clock) {
        if clock.state() == Tick {
            self.state_past = self.state_new;
            self.state_new = a
        }
    }

    pub fn output(self, clock: &Clock) -> Bit {
        if clock.state() == Tick {
            self.state_past
        } else {
            self.state_new
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::ClockState::{Tick, Tock};
    use crate::logic::Bit::{I, O};

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
        //Tock
        clock.next();

        // nothing happen
        dff.input(I, &clock);
        // output new (=O)
        assert_eq!(dff.output(&clock), O);
    }
}