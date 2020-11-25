#![allow(dead_code, non_snake_case)]

use crate::logic::Word;
use crate::logic::{bit, DMux, Mux};
use crate::logic::bit::{I, O};
use crate::sequential::ClockState::{Tick, Tock};
use crate::sequential::{Clock, RAM4K};

use std::io;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct CPU {
    // 
}

#[derive(Debug, Copy, Clone)]
pub struct ROM32K {
    //
}

#[derive(Copy, Clone)]
pub struct Screen {
    rams: [RAM4K; 2]
}

impl Screen {
    pub fn new() -> Self {
        Screen { rams: [RAM4K::new(); 2] }
    }

    pub fn input(&mut self, clock: &Clock, input: Word, address: [bit; 13], load: bit) {
        let bits = [
            DMux(input[0], address[12]),
            DMux(input[1], address[12]),
            DMux(input[2], address[12]),
            DMux(input[3], address[12]),
            DMux(input[4], address[12]),
            DMux(input[5], address[12]),
            DMux(input[6], address[12]),
            DMux(input[7], address[12]),
            DMux(input[8], address[12]),
            DMux(input[9], address[12]),
            DMux(input[10], address[12]),
            DMux(input[11], address[12]),
            DMux(input[12], address[12]),
            DMux(input[13], address[12]),
            DMux(input[14], address[12]),
            DMux(input[15], address[12]),
        ];
        for i in 0..2 {
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
                        address[6], address[7], address[8], address[9], address[10], address[11]], load)
        }
    }

    pub fn output(&self, clock: &Clock, address: [bit; 13]) -> Word {
        let output1 = self.rams[0].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5],
                                                               address[6], address[7], address[8], address[9], address[10], address[11]]);
        let output2 = self.rams[1].output(clock, [address[0], address[1], address[2], address[3], address[4], address[5],
                                                                address[6], address[7], address[8], address[9], address[10], address[11]]);
        // Draw screen
        Word::new([
            Mux(output1[0], output2[0], address[12]),
            Mux(output1[1], output2[1], address[12]),
            Mux(output1[2], output2[2], address[12]),
            Mux(output1[3], output2[3], address[12]),
            Mux(output1[4], output2[4], address[12]),
            Mux(output1[5], output2[5], address[12]),
            Mux(output1[6], output2[6], address[12]),
            Mux(output1[7], output2[7], address[12]),
            Mux(output1[8], output2[8], address[12]),
            Mux(output1[9], output2[9], address[12]),
            Mux(output1[10], output2[10], address[12]),
            Mux(output1[11], output2[11], address[12]),
            Mux(output1[12], output2[12], address[12]),
            Mux(output1[13], output2[13], address[12]),
            Mux(output1[14], output2[14], address[12]),
            Mux(output1[15], output2[15], address[12]),
        ])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Keyboard {
    word: Word
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { word: Word::new([O; 16])}
    }

    // This function work only when Tick start
    pub fn input(&mut self, clock: &Clock) {
        if clock.state() == Tick {
            let stdin = io::stdin();
            for line_result in stdin.lock().lines() {
                let line = line_result.expect("line error");
                if let Some(word) = Keyboard::matching(line) {
                    self.word = word;
                    break;
                }
            }
        }
    }

    fn matching(line: String) -> Option<Word>{
        match line.as_bytes() {
            // nothing
            [] => Some(Word::new([O; 16])),

            // 0 ~ 9
            [48] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O])),
            [49] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, I])),
            [50] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, O, I, O])),
            [51] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, O, I, I])),
            [52] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, I, O, O])),
            [53] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, I, O, I])),
            [54] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, I, I, O])),
            [55] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, I, I, I])),
            [56] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, I, O, O, O])),
            [57] => Some(Word::new([O, O, O, O, O, O, O, O, O, O, I, I, I, O, O, I])),

            // A ~ Z
            [65] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, O, O, I])),
            [66] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, O, I, O])),
            [67] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, O, I, I])),
            [68] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, I, O, O])),
            [69] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, I, O, I])),
            [70] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, I, I, O])),
            [71] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, O, I, I, I])),
            [72] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, O, O, O])),
            [73] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, O, O, I])),
            [74] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, O, I, O])),
            [75] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, O, I, I])),
            [76] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, I, O, O])),
            [77] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, I, O, I])),
            [78] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, I, I, O])),
            [79] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, O, I, I, I, I])),
            [80] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, O, O, O])),
            [81] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, O, O, I])),
            [82] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, O, I, O])),
            [83] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, O, I, I])),
            [84] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, I, O, O])),
            [85] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, I, O, I])),
            [86] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, I, I, O])),
            [87] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, O, I, I, I])),
            [88] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, I, O, O, O])),
            [89] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, I, O, O, I])),
            [90] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, O, I, I, O, I, O])),

            // a ~ z
            [97] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, O, O, I])),
            [98] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, O, I, O])),
            [99] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, O, I, I])),
            [100] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, I, O, O])),
            [101] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, I, O, I])),
            [102] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, I, I, O])),
            [103] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, O, I, I, I])),
            [104] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, O, O, O])),
            [105] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, O, O, I])),
            [106] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, O, I, O])),
            [107] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, O, I, I])),
            [108] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, I, O, O])),
            [109] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, I, O, I])),
            [110] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, I, I, O])),
            [111] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, O, I, I, I, I])),
            [112] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, O, O, O])),
            [113] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, O, O, I])),
            [114] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, O, I, O])),
            [115] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, O, I, I])),
            [116] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, I, O, O])),
            [117] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, I, O, I])),
            [118] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, I, I, O])),
            [119] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, O, I, I, I])),
            [120] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, I, O, O, O])),
            [121] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, I, O, O, I])),
            [122] => Some(Word::new([O, O, O, O, O, O, O, O, O, I, I, I, I, O, I, O])),

            // newline
            [110, 101, 119, 108, 105, 110, 101] 
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, O, O])),
            // backspace
            [98, 97, 99, 107, 115, 112, 97, 99, 101]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, O, I])),
            // leftarrow
            [27, 91, 68]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, O])), 
            [108, 101, 102, 116, 97, 114, 114, 111, 119]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, O])), 
            // uparrow
            [27, 91, 65]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, I])),
            [117, 112, 97, 114, 114, 111, 119]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, I])),
            // rightarrow
            [27, 91, 67]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, O])),
            [114, 105, 103, 104, 116, 97, 114, 114, 111, 119]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, O])),
            // downarrow
            [27, 91, 66]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, I])),
            [100, 111, 119, 110, 97, 114, 114, 111, 119]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, I])),
            // home
            [32]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, I, O])),
            [104, 111, 109, 101]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, I, O])),
            // end
            [101, 110, 100]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, I, I])),
            // pageup
            [112, 97, 103, 101, 117, 112]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, O, O])),
            // pagedown
            [112, 97, 103, 101, 100, 111, 119, 110]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, O, I])),
            // insert
            [105, 110, 115, 101, 114, 116]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, O])),
            // delete
            [100, 101, 108, 101, 116, 101]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, O])),
            // esc
            [27]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, I])),
            [101, 115, 99]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, I])),
            // f1
            [27, 79, 80]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, O])),
            [102, 49]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, O])),
            [70, 49]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, O])),
            // f2
            [27, 79, 81]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, I])),
            [102, 50]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, I])),
            [70, 50]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, I])),
            // f3
            [27, 79, 82]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, O])),
            [102, 51]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, O])),
            [70, 51]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, O])),
            // f4
            [27, 79, 83]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, I])),
            [102, 52]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, I])),
            [70, 52]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, I])),
            // f5
            [27, 91, 49, 53, 126]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, O])),
            [102, 53]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, O])),
            [70, 53]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, O])),
            // f6
            [27, 91, 49, 55, 126]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, I])),
            [102, 54]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, I])),
            [70, 54]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, I])),
            // f7
            [27, 91, 49, 56, 126]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, I, I])),
            [102, 55]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, I, I])),
            [70, 55]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, I, I])),
            // f8
            [27, 91, 49, 57, 126]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, O])),
            [102, 56]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, O])),
            [70, 56]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, O])),
            // f9
            [27, 91, 50, 48, 126]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, I])),
            [102, 57]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, I])),
            [70, 57]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, I])),
            // f10
            [27, 91, 50, 49, 126]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, O])),
            [102, 49, 48]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, O])),
            [70, 49, 48]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, O])),
            // f11
            [102, 49, 49]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, I])),
            [70, 49, 49]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, I])),
            // f12
            [102, 49, 50]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, I, O, O, O])),
            [70, 49, 50]
                => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, I, O, O, O])),
             
            // Input error
            _ => None
        }
    }

    pub fn output(&self) -> Word {
        self.word
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Memory {
    //
}

#[derive(Debug, Copy, Clone)]
pub struct Computer {
    //
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::prelude::*;
    use super::*;

    #[test]
    fn for_cpu() {
        unimplemented!()
    }

    #[test]
    fn for_computer() {
        unimplemented!()
    }

    #[test]
    fn for_keyboard_0() {
        // Imput '0'!
        let stdin = io::stdin();
        for line_result in stdin.lock().lines() {
            let line = line_result.expect("line error");
            if let Some(word) = Keyboard::matching(line) {
                assert_eq!(
                    word,
                    Word::new([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O])
                );
                break;
            }
        }
    }

    #[test]
    fn for_memory() {
        unimplemented!()
    }

    #[test]
    fn for_rom32k() {
        unimplemented!()
    }

    #[test]
    fn for_screen() {
        // initialize as past: O, new: O
        let mut ram = Screen::new();
        // initialize state as Tick
        let mut clock = Clock::new();

        let word_i = Word::new([I, O, I, O, I, I, O, O, O, I, O, I, O, O, I, I]);
        let word_o = Word::new([O, I, O, I, O, O, I, I, I, O, I, O, I, I, O, O]);
        let word_0 = Word::new([O; 16]);

        // input as past: word_0, new: word_i in registers
        ram.input(&clock, word_i, [O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output past in register
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]), word_0);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O], O);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: I
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O], O);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tick
        clock.next();

        // initialize as past: I, new: O
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output past
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]), word_i);

        // Tock
        clock.next();

        // nothing happened
        ram.input(&clock, word_o, [O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        // output new
        assert_eq!(ram.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]), word_o);
        
        clock.next();

        ram.input(&clock, word_o, [O, O, I, O, O, I, O, O, I, O, O, I, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O]), word_0);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O], O);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O]), word_o);

        clock.next();

        ram.input(&clock, word_i, [O, O, I, O, O, I, O, O, I, O, O, I, O], I);
        assert_eq!(ram.output(&clock, [O, O, I, O, O, I, O, O, I, O, O, I, O]), word_i);

        clock.next();
    }
}