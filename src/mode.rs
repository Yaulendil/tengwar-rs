//! This module consolidates the modes of transliteration for simplicity.

pub mod beleriand;
// pub mod general;
pub mod gondor;
pub mod quenya;

mod iter;

pub use beleriand::Beleriand;
// pub use general::General;
pub use gondor::Gondor;
pub use quenya::Quenya;

use crate::{Numeral, Token, TokenIter};


#[derive(Clone, Debug)]
pub enum ParseAction {
    /// Nothing could be done with the input.
    MatchedNone,
    /// A portion of the input was processed successfully, but more is needed.
    MatchedPart(usize),
    /// A complete [`Token`] has been processed.
    MatchedToken {
        token: Token,
        len: usize,
    },
    /// A portion of the input has been determined to be irrelevant to the
    ///     processing. A number of `char`s should be passed through unchanged.
    Skip(usize),
    Escape,
}


pub trait TengwarMode: Default + Sized {
    const MAX_CHUNK: usize = 3;

    fn mode_iter(data: Vec<char>) -> iter::ModeIter<Self> {
        iter::ModeIter::new(data)
    }

    fn token_iter(input: impl AsRef<str>) -> TokenIter<iter::ModeIter<Self>> {
        let data: Vec<char> = input.as_ref().chars().collect();
        Self::mode_iter(data).into()
    }

    fn tokens(input: impl AsRef<str>) -> Vec<Token> {
        Self::token_iter(input).collect()
    }

    fn find_numeral(&mut self, slice: &[char]) -> Option<(Numeral, usize)> {
        Numeral::parse(slice)
    }

    fn finish_current(&mut self) -> Option<Token>;

    fn process(&mut self, chunk: &[char]) -> ParseAction;
}


#[test]
fn test_iter() {
    let iter = quenya::Quenya2::token_iter("yéni 144");
    let text = iter.collect::<String>();
    eprintln!("{}", text);
}
