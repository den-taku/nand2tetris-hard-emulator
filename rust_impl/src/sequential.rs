#![allow(dead_code, non_snake_case)]

use crate::ClockState::{Tick, Tock};
use crate::logic::{bit, bit::{O, I}, *};
use crate::arithmetic::Add16;

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

// must use input and output both
// when use, save input value and output one separately in variables
// use input first
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

// must use input and output both
// when use, save input value and output one separately in variables
// use input first
#[derive(Debug, Copy, Clone)]
pub struct RAM8 {
    registers: [Register; 8]
}

impl RAM8 {
    pub fn new() -> Self {
        RAM8 { registers: [Register::new(); 8]}
    }

    pub fn input(&mut self, clock: &Clock, input: Word, address: [bit; 3], load: bit) {
        let bits = [
            DMux8Way(input[0], address),
            DMux8Way(input[1], address),
            DMux8Way(input[2], address),
            DMux8Way(input[3], address),
            DMux8Way(input[4], address),
            DMux8Way(input[5], address),
            DMux8Way(input[6], address),
            DMux8Way(input[7], address),
            DMux8Way(input[8], address),
            DMux8Way(input[9], address),
            DMux8Way(input[10], address),
            DMux8Way(input[11], address),
            DMux8Way(input[12], address),
            DMux8Way(input[13], address),
            DMux8Way(input[14], address),
            DMux8Way(input[15], address),
        ];
        for i in 0..8 {
            self.registers[i].input(clock, Word::new([
                bits[0][i],
                bits[1][i],
                bits[2][i],
                bits[3][i],
                bits[4][i],
                bits[5][i],
                bits[6][i],
                bits[7][i],
                bits[8][i],
                bits[9][i],
                bits[10][i],
                bits[11][i],
                bits[12][i],
                bits[13][i],
                bits[14][i],
                bits[15][i],
            ]), load);
        }
    }

    pub fn output(&self, clock: &Clock, address: [bit; 3]) -> Word {
        Mux8Way16(
            self.registers[0].output(clock), 
            self.registers[1].output(clock), 
            self.registers[2].output(clock), 
            self.registers[3].output(clock), 
            self.registers[4].output(clock), 
            self.registers[5].output(clock), 
            self.registers[6].output(clock), 
            self.registers[7].output(clock), 
            address
        )
    }
}

#[derive(Copy, Clone)]
pub struct RAM64 {
    rams: [RAM8; 8]
}

impl RAM64 {
    pub fn new() -> Self {
        RAM64 { rams: [RAM8::new(); 8] }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, address: [bit; 6], load: bit) {
        let bits = [
            DMux8Way(input[0], [address[3], address[4], address[5]]),
            DMux8Way(input[1], [address[3], address[4], address[5]]),
            DMux8Way(input[2], [address[3], address[4], address[5]]),
            DMux8Way(input[3], [address[3], address[4], address[5]]),
            DMux8Way(input[4], [address[3], address[4], address[5]]),
            DMux8Way(input[5], [address[3], address[4], address[5]]),
            DMux8Way(input[6], [address[3], address[4], address[5]]),
            DMux8Way(input[7], [address[3], address[4], address[5]]),
            DMux8Way(input[8], [address[3], address[4], address[5]]),
            DMux8Way(input[9], [address[3], address[4], address[5]]),
            DMux8Way(input[10], [address[3], address[4], address[5]]),
            DMux8Way(input[11], [address[3], address[4], address[5]]),
            DMux8Way(input[12], [address[3], address[4], address[5]]),
            DMux8Way(input[13], [address[3], address[4], address[5]]),
            DMux8Way(input[14], [address[3], address[4], address[5]]),
            DMux8Way(input[15], [address[3], address[4], address[5]]),
        ];
        for i in 0..8 {
            self.rams[i].input(clock, Word::new([
                bits[0][i],
                bits[1][i],
                bits[2][i],
                bits[3][i],
                bits[4][i],
                bits[5][i],
                bits[6][i],
                bits[7][i],
                bits[8][i],
                bits[9][i],
                bits[10][i],
                bits[11][i],
                bits[12][i],
                bits[13][i],
                bits[14][i],
                bits[15][i],
            ]), [address[0], address[1], address[2]], load);
        }
    }

    pub fn output(&self, clock: &Clock, address: [bit; 6]) -> Word {
        Mux8Way16(
            self.rams[0].output(clock, [address[0], address[1], address[2]]), 
            self.rams[1].output(clock, [address[0], address[1], address[2]]), 
            self.rams[2].output(clock, [address[0], address[1], address[2]]), 
            self.rams[3].output(clock, [address[0], address[1], address[2]]), 
            self.rams[4].output(clock, [address[0], address[1], address[2]]), 
            self.rams[5].output(clock, [address[0], address[1], address[2]]), 
            self.rams[6].output(clock, [address[0], address[1], address[2]]), 
            self.rams[7].output(clock, [address[0], address[1], address[2]]), 
            [address[3], address[4], address[5]]
        )
    }
}

#[derive(Copy, Clone)]
pub struct RAM512 {
    rams: [RAM64; 8]
}

impl RAM512 {
    pub fn new() -> Self {
        RAM512 { rams: [RAM64::new(); 8] }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, address: [bit; 9], load: bit) {
        let bits = [
            DMux8Way(input[0], [address[6], address[7], address[8]]),
            DMux8Way(input[1], [address[6], address[7], address[8]]),
            DMux8Way(input[2], [address[6], address[7], address[8]]),
            DMux8Way(input[3], [address[6], address[7], address[8]]),
            DMux8Way(input[4], [address[6], address[7], address[8]]),
            DMux8Way(input[5], [address[6], address[7], address[8]]),
            DMux8Way(input[6], [address[6], address[7], address[8]]),
            DMux8Way(input[7], [address[6], address[7], address[8]]),
            DMux8Way(input[8], [address[6], address[7], address[8]]),
            DMux8Way(input[9], [address[6], address[7], address[8]]),
            DMux8Way(input[10], [address[6], address[7], address[8]]),
            DMux8Way(input[11], [address[6], address[7], address[8]]),
            DMux8Way(input[12], [address[6], address[7], address[8]]),
            DMux8Way(input[13], [address[6], address[7], address[8]]),
            DMux8Way(input[14], [address[6], address[7], address[8]]),
            DMux8Way(input[15], [address[6], address[7], address[8]]),
        ];
        for i in 0..8 {
            self.rams[i].input(clock, Word::new([
                bits[0][i],
                bits[1][i],
                bits[2][i],
                bits[3][i],
                bits[4][i],
                bits[5][i],
                bits[6][i],
                bits[7][i],
                bits[8][i],
                bits[9][i],
                bits[10][i],
                bits[11][i],
                bits[12][i],
                bits[13][i],
                bits[14][i],
                bits[15][i],
            ]), [address[0], address[1], address[2], address[3], address[4], address[5]], load);
        } 
    }

    pub fn output(&self, clock: &Clock, address: [bit; 9]) -> Word {
        Mux8Way16(
            self.rams[0].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[1].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[2].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[3].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[4].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[5].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[6].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            self.rams[7].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5]]), 
            [address[6], address[7], address[8]]
        )
    }
}

#[derive(Copy, Clone)]
pub struct RAM4K {
    rams: [RAM512; 8]
}

impl RAM4K {
    pub fn new() -> Self {
        RAM4K { rams: [RAM512::new(); 8] }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, address: [bit; 12], load: bit) {
        let bits = [
            DMux8Way(input[0], [address[9], address[10], address[11]]),
            DMux8Way(input[1], [address[9], address[10], address[11]]),
            DMux8Way(input[2], [address[9], address[10], address[11]]),
            DMux8Way(input[3], [address[9], address[10], address[11]]),
            DMux8Way(input[4], [address[9], address[10], address[11]]),
            DMux8Way(input[5], [address[9], address[10], address[11]]),
            DMux8Way(input[6], [address[9], address[10], address[11]]),
            DMux8Way(input[7], [address[9], address[10], address[11]]),
            DMux8Way(input[8], [address[9], address[10], address[11]]),
            DMux8Way(input[9], [address[9], address[10], address[11]]),
            DMux8Way(input[10], [address[9], address[10], address[11]]),
            DMux8Way(input[11], [address[9], address[10], address[11]]),
            DMux8Way(input[12], [address[9], address[10], address[11]]),
            DMux8Way(input[13], [address[9], address[10], address[11]]),
            DMux8Way(input[14], [address[9], address[10], address[11]]),
            DMux8Way(input[15], [address[9], address[10], address[11]]),
        ];
        for i in 0..8 {
            self.rams[i].input(clock, Word::new([
                bits[0][i],
                bits[1][i],
                bits[2][i],
                bits[3][i],
                bits[4][i],
                bits[5][i],
                bits[6][i],
                bits[7][i],
                bits[8][i],
                bits[9][i],
                bits[10][i],
                bits[11][i],
                bits[12][i],
                bits[13][i],
                bits[14][i],
                bits[15][i],
            ]), [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]], load);
        } 
    }

    pub fn output(&self, clock: &Clock, address: [bit; 12]) -> Word {
        Mux8Way16(
            self.rams[0].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[1].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[2].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[3].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[4].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[5].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[6].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            self.rams[7].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]]), 
            [address[9], address[10], address[11]]
        )
    }
}

#[derive(Copy, Clone)]
pub struct RAM16K {
    rams: [RAM4K; 4]
}

impl RAM16K {
    pub fn new() -> Self {
        RAM16K { rams: [RAM4K::new(); 4] }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, address: [bit; 14], load: bit) {
        let bits = [
            DMux4Way(input[0], [address[12], address[13]]),
            DMux4Way(input[1], [address[12], address[13]]),
            DMux4Way(input[2], [address[12], address[13]]),
            DMux4Way(input[3], [address[12], address[13]]),
            DMux4Way(input[4], [address[12], address[13]]),
            DMux4Way(input[5], [address[12], address[13]]),
            DMux4Way(input[6], [address[12], address[13]]),
            DMux4Way(input[7], [address[12], address[13]]),
            DMux4Way(input[8], [address[12], address[13]]),
            DMux4Way(input[9], [address[12], address[13]]),
            DMux4Way(input[10], [address[12], address[13]]),
            DMux4Way(input[11], [address[12], address[13]]),
            DMux4Way(input[12], [address[12], address[13]]),
            DMux4Way(input[13], [address[12], address[13]]),
            DMux4Way(input[14], [address[12], address[13]]),
            DMux4Way(input[15], [address[12], address[13]]),
        ];
        for i in 0..4 {
            self.rams[i].input(clock, Word::new([
                bits[0][i],
                bits[1][i],
                bits[2][i],
                bits[3][i],
                bits[4][i],
                bits[5][i],
                bits[6][i],
                bits[7][i],
                bits[8][i],
                bits[9][i],
                bits[10][i],
                bits[11][i],
                bits[12][i],
                bits[13][i],
                bits[14][i],
                bits[15][i],
            ]), [address[0], address[1], address[2], address[3], address[4], address[5], 
                address[6], address[7], address[8], address[9], address[10], address[11]], load);
        } 
    }

    pub fn output(&self, clock: &Clock, address: [bit; 14]) -> Word {
        Mux4Way16(
            self.rams[0].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], 
                                         address[6], address[7], address[8], address[9], address[10], address[11]]), 
            self.rams[1].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], 
                                         address[6], address[7], address[8], address[9], address[10], address[11]]), 
            self.rams[2].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], 
                                         address[6], address[7], address[8], address[9], address[10], address[11]]), 
            self.rams[3].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5], 
                                         address[6], address[7], address[8], address[9], address[10], address[11]]), 
            [address[12], address[13]]
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PC {
    counter: Register
}

impl PC {
    pub fn new() -> Self {
        PC { counter: Register::new() }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, inc: bit, load: bit, reset: bit) {
        let clock_tmp = match clock.state() {
            Tick => {
                let mut c = Clock::new();
                c.next();
                c
            },
            Tock => Clock::new()
        };
        self.counter.input(clock, 
            Mux16(
                Mux16(
                    Mux16(
                        self.output(&clock_tmp),
                        Add16(
                            self.output(&clock_tmp), 
                            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I])
                        ),
                        inc
                    ),
                    input,
                    load
                ),
                Word::new([O; 16]),
                reset
            ),
             I
        );
    }

    pub fn output(&self, clock: &Clock) -> Word {
        self.counter.output(clock)
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

    #[test]
    fn for_ram8() {
        // initialize as past: O, new: O
        let mut ram8 = RAM8::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
        let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
        let word_0 = Word::new([O; 16]);

        // input as past: word_0, new: word_i in registers
        ram8.input(&clock, word_i, [O, O, O], I);
        // output past in register
        assert_eq!(ram8.output(&clock, [O, O, O]), word_0);

        // Tock
        clock.next();

        // nothing happened
        ram8.input(&clock, word_o, [O, O, O], O);
        // output new
        assert_eq!(ram8.output(&clock, [O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        ram8.input(&clock, word_o, [O, O, O], O);
        // output past
        assert_eq!(ram8.output(&clock, [O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram8.input(&clock, word_o, [O, O, O], I);
        // output new
        assert_eq!(ram8.output(&clock, [O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        ram8.input(&clock, word_o, [O, O, O], I);
        // output past
        assert_eq!(ram8.output(&clock, [O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram8.input(&clock, word_o, [O, O, O], I);
        // output new
        assert_eq!(ram8.output(&clock, [O, O, O]), word_o);
        
        clock.next();

        ram8.input(&clock, word_o, [O, O, I], I);
        assert_eq!(ram8.output(&clock, [O, O, I]), word_0);

        clock.next();

        ram8.input(&clock, word_i, [O, O, I], O);
        assert_eq!(ram8.output(&clock, [O, O, I]), word_o);

        clock.next();

        ram8.input(&clock, word_i, [O, O, I], O);
        assert_eq!(ram8.output(&clock, [O, O, I]), word_o);

        clock.next();

        ram8.input(&clock, word_i, [O, O, I], I);
        assert_eq!(ram8.output(&clock, [O, O, I]), word_o);

        clock.next();

        ram8.input(&clock, word_i, [O, O, I], I);
        assert_eq!(ram8.output(&clock, [O, O, I]), word_o);

        clock.next();

        ram8.input(&clock, word_i, [O, O, I], I);
        assert_eq!(ram8.output(&clock, [O, O, I]), word_i);

        clock.next();
    }

    #[test]
    fn for_ram64() {
        // initialize as past: O, new: O
        let mut ram = RAM64::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
        let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
        let word_0 = Word::new([O; 16]);

        // input as past: word_0, new: word_i in registers
        ram.input(&clock, word_i, [O, O, O, O, O, O], I);
        // output past in register
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O]), word_0);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O], O);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        ram.input(&clock, word_o, [O, O, O, O, O, O], O);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        ram.input(&clock, word_o, [O, O, O, O, O, O], I);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O]), word_o);
        
        clock.next();

        ram.input(&clock, word_o, [O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I]), word_0);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I]), word_i);

        clock.next();
    }

    #[test]
    fn for_ram512() {
        // initialize as past: O, new: O
        let mut ram = RAM512::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
        let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
        let word_0 = Word::new([O; 16]);

        // input as past: word_0, new: word_i in registers
        ram.input(&clock, word_i, [O, O, O, O, O, O, O, O, O], I);
        // output past in register
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O]), word_0);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O], O);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O], O);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O], I);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O]), word_o);
        
        clock.next();

        ram.input(&clock, word_o, [O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I]), word_0);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I]), word_i);

        clock.next();
    }

    #[test]
    fn for_ram4k() {
        // initialize as past: O, new: O
        let mut ram = RAM4K::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
        let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
        let word_0 = Word::new([O; 16]);

        // input as past: word_0, new: word_i in registers
        ram.input(&clock, word_i, [O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output past in register
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O]), word_0);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O], O);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O], O);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O]), word_o);
        
        clock.next();

        ram.input(&clock, word_o, [O, O, I, O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I]), word_0);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I]), word_i);

        clock.next();
    }

    #[test]
    fn for_ram16k() {
        // initialize as past: O, new: O
        let mut ram = RAM16K::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
        let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
        let word_0 = Word::new([O; 16]);

        // input as past: word_0, new: word_i in registers
        ram.input(&clock, word_i, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output past in register
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_0);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], O);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], O);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O]), word_o);
        
        clock.next();

        ram.input(&clock, word_o, [O, O, I, O, O, I, O, O, I, O, O, I, O, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O, O]), word_0);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O, O], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O, O], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O, O]), word_i);

        clock.next();
    }

    #[test]
    fn for_pc() {
        let mut clock = Clock::new();
        let mut pc = PC::new();

        let word0 = Word::new([O; 16]);
        let word1 = Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]);
        let input = Word::new([O, O, I, O, O, I, O, O, I, O, O, I, O, O, I, O]);

        pc.input(&clock, word0, I, O, O);
        assert_eq!(pc.output(&clock), word0);

        clock.next(); // Tock

        pc.input(&clock, word0, I, O, O);
        assert_eq!(pc.output(&clock), Add16(word0, word1));

        clock.next(); // Tick

        pc.input(&clock, word0, I, O, O);
        assert_eq!(pc.output(&clock), Add16(word0, word1));

        clock.next(); // Tock

        pc.input(&clock, word0, I, O, O);
        assert_eq!(pc.output(&clock), Add16(Add16(word0, word1), word1));

        clock.next(); // Tick

        pc.input(&clock, word0, O, O, I);
        assert_eq!(pc.output(&clock), Add16(word1, word1));

        clock.next(); // Tock

        pc.input(&clock, input, O, I, O);
        assert_eq!(pc.output(&clock), word0);

        clock.next(); // Tick

        pc.input(&clock, input, O, I, O);
        assert_eq!(pc.output(&clock), word0);

        clock.next(); // Tock

        pc.input(&clock, word0, O, O, O);
        assert_eq!(pc.output(&clock), input);

        clock.next(); // Tick

        pc.input(&clock, word0, O, O, O);
        assert_eq!(pc.output(&clock), input);

        clock.next(); // Tock

        pc.input(&clock, word0, O, O, O);
        assert_eq!(pc.output(&clock), input);
    }
}