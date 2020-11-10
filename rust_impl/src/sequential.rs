#![allow(dead_code, non_snake_case)]

use crate::ClockState::{Tick, Tock};

// tick: input, update internal state
// tock: output 
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClockState {
    Tick,
    Tock
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


#[cfg(test)]
mod tests {
    use super::*;
    use super::ClockState::{Tick, Tock};

    #[test]
    fn for_clock_new() {
        let mut clock = Clock::new();
        assert_eq!(clock.state(), Tick);
        clock.next();
        assert_eq!(clock.state(), Tock);
        clock.next();
        assert_eq!(clock.state(), Tick);
    }

    //
}