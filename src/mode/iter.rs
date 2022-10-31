use crate::{characters::{Numeral, punctuation}, Token};
use super::{ParseAction, TengwarMode};


/// The result of a single "step" of a [`ModeIter`]. Multiple steps can be
///     performed for each iteration.
#[derive(Clone, Debug)]
enum IterStep {
    /// The iteration is not complete. Another step should be run immediately.
    Again,
    /// The [`ModeIter`] is exhausted. The iteration can safely return [`None`].
    Empty,
    /// The iteration is complete. The [`Token`] should now be returned.
    Success(Token),
}


#[derive(Debug)]
pub struct ModeIter<M: TengwarMode> {
    data: Vec<char>,
    head: usize,
    size: usize,
    skip: usize,
    pub mode: M,
}

impl<M: TengwarMode> ModeIter<M> {
    pub fn new(data: Vec<char>) -> Self {
        let size: usize = data.len().min(M::MAX_CHUNK);
        Self { data, head: 0, size, skip: 0, mode: M::default() }
    }

    fn advance_head(&mut self, n: usize) {
        self.head += n;
        self.size = self.data.len().min(M::MAX_CHUNK);
    }

    fn parse_numeral(&self) -> Option<(Numeral, usize)> {
        Numeral::parse(&self.data[self.head..])
    }

    fn skip_one(&mut self) -> char {
        let here: char = self.data[self.head];
        self.advance_head(1);
        here
    }

    fn step(&mut self) -> IterStep {
        let len: usize = self.data.len();

        if self.head < len {
            //  Obey the "skip" counter above all else.
            if 0 < self.skip {
                if let Some(token) = self.mode.finish_current() {
                    self.advance_head(0);
                    IterStep::Success(token)
                } else {
                    self.skip -= 1;
                    IterStep::Success(Token::Char(self.skip_one()))
                }
            }

            //  Delegate matching to the Mode implementation.
            else if 0 < self.size { // skip == 0
                let Self { ref data, head, size, ref mut mode, .. } = *self;

                let end: usize = len.min(head + size);
                let chunk: &[char] = &data[head..end];

                match mode.process(chunk) {
                    ParseAction::MatchedNone => {
                        self.size -= 1;
                        IterStep::Again
                    }
                    ParseAction::MatchedPart(n) => {
                        self.advance_head(n);
                        IterStep::Again
                    }
                    ParseAction::MatchedToken { token, len } => {
                        self.advance_head(len);
                        IterStep::Success(token)
                    }
                    ParseAction::Skip(n) => {
                        self.skip += n;

                        match self.mode.finish_current() {
                            Some(token) => IterStep::Success(token),
                            None => IterStep::Again,
                        }
                    }
                    ParseAction::Escape => {
                        self.advance_head(1);
                        self.skip += 1;
                        IterStep::Again
                    }
                }
            }

            //  Nothing more can be added. If a tengwa is currently being
            //      constructed, finalize and return it.
            else if let Some(token) = self.mode.finish_current() {
                self.advance_head(0);
                IterStep::Success(token)
            }

            //  Look for a numeric value.
            else if let Some((num, len)) = self.parse_numeral() {
                self.advance_head(len);
                IterStep::Success(Token::Number(num))
            }

            else if let Some(punct) = punctuation(self.data[self.head]) {
                self.advance_head(1);
                IterStep::Success(Token::Char(punct))
            }

            //  Give up and pass the current char through unchanged.
            else {
                IterStep::Success(Token::Char(self.skip_one()))
            }
        } else { // len <= head
            match self.mode.finish_current() {
                Some(token) => IterStep::Success(token),
                None => IterStep::Empty,
            }
        }
    }
}

impl<M: TengwarMode> Iterator for ModeIter<M> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.step() {
                IterStep::Again => continue,
                IterStep::Empty => break None,
                IterStep::Success(token) => break Some(token),
            }
        }
    }
}
