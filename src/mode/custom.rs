#![cfg(feature = "mode-custom")]

use std::collections::HashMap;
use super::*;


#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Consonant {
    pub char: char,
}


#[derive(Deserialize, Serialize)]
pub struct CustomMode {
    pub chunks: usize,
    pub vowels_first: bool,

    pub consonants: HashMap<String, Consonant>,
}

impl TengwarMode for CustomMode {
    fn finish_current(&mut self) -> Option<Token> {
        todo!()
    }

    fn process(&mut self, _chunk: &[char]) -> ParseAction {
        todo!()
    }
}
