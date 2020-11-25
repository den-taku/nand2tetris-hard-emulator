#![allow(dead_code, non_snake_case)]

use crate::logic::Word;
use crate::logic::bit::{I, O};
use crate::sequential::ClockState::{Tick, Tock};
use crate::sequential::Clock;

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

#[derive(Debug, Copy, Clone)]
pub struct Screen {
    //
}

#[derive(Debug, Copy, Clone)]
pub struct Keyboard {
    word: Word
}

impl Keyboard {
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
        unimplemented!()
    }
}