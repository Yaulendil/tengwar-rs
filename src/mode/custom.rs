pub mod file;

use std::collections::HashMap;
use crate::characters::*;
use super::*;
use file::*;


/// A runtime-defined mode of the Tengwar.
#[derive(Clone, Debug)]
pub struct CustomMode {
    pub chunks: usize,
    pub vowels_first: bool,

    pub checks_mod: Vec<Check>,
    pub checks_new: Vec<Check>,

    pub consonants: HashMap<Vec<char>, GlyphSpec>,
    pub vowels: HashMap<Vec<char>, TehtaSpec>,

    current: Option<Glyph>,
    previous: Option<Glyph>,
}

impl CustomMode {
    pub fn get_consonant(&self, _chunk: &[char]) -> Option<Glyph> {
        todo!()
    }

    pub fn get_diphthong(&self, _chunk: &[char]) -> Option<Glyph> {
        todo!()
    }

    pub fn get_vowel(&self, _chunk: &[char]) -> Option<Glyph> {
        todo!()
    }
}

impl TengwarMode for CustomMode {
    fn finish_current(&mut self) -> Option<Token> {
        self.previous = self.current.take();
        self.previous.map(Token::Glyph)
    }

    fn process(&mut self, chunk: &[char]) -> ParseAction {
        macro_rules! _finish {
            ($glyph:expr) => {finish!($glyph, 0)};
            ($glyph:expr, $len:expr) => {{
                let glyph = $glyph;

                self.current = None;
                self.previous = Some(glyph);

                ParseAction::MatchedToken {
                    token: Token::Glyph(glyph),
                    len: $len,
                }
            }};
        }

        let _initial: bool = self.previous.is_none();

        if let ['\\', _, ..] = chunk {
            ParseAction::ESC_BACKSLASH
        } else if let Some(_current) = &mut self.current {
            //  A glyph is currently being constructed. Try to continue it.
            for check in &self.checks_mod {
                match check {
                    Check::Consonant => {}
                    Check::Diphthong => {}
                    Check::Vowel => {}
                    Check::Rince => {}
                    Check::Labial => {}
                    Check::Nasal => {}
                    Check::Palatal => {}
                    Check::Replacements => {}
                }
            }

            ParseAction::MatchedNone
        } else {
            //  Try to find a new glyph.
            for check in &self.checks_new {
                match check {
                    Check::Consonant => {}
                    Check::Diphthong => {}
                    Check::Vowel => {}
                    Check::Rince => {}
                    Check::Labial => {}
                    Check::Nasal => {}
                    Check::Palatal => {}
                    Check::Replacements => {}
                }
            }

            ParseAction::MatchedNone
        }
    }
}
