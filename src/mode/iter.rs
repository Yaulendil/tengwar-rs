use crate::{Numeral, Token};
use super::{ParseAction, TengwarMode};


/// The result of a single "tick" of a [`ModeIter`]. Multiple ticks can be
///     performed for each iteration.
#[derive(Clone, Debug)]
enum IterTick {
    /// The [`ModeIter`] is exhausted. The iteration can safely return [`None`].
    Empty,
    /// The iteration is not complete. Another tick should be run immediately.
    Retry,
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

    fn tick(&mut self) -> IterTick {
        let len: usize = self.data.len();

        if self.head < len {
            //  Obey the "skip" counter above all else.
            if 0 < self.skip {
                self.skip -= 1;
                IterTick::Success(Token::Char(self.skip_one()))
            }

            //  Delegate matching to the Mode implementation.
            else if 0 < self.size { // skip == 0
                let Self { ref data, head, size, ref mut mode, .. } = *self;

                let end: usize = len.min(head + size);
                let chunk: &[char] = &data[head..end];

                match mode.process(chunk) {
                    ParseAction::MatchedNone => {
                        self.size -= 1;
                        IterTick::Retry
                    }
                    ParseAction::MatchedPart(n) => {
                        self.head += n;
                        IterTick::Retry
                    }
                    ParseAction::MatchedToken { token, len } => {
                        self.advance_head(len);
                        IterTick::Success(token)
                    }
                    ParseAction::Skip(n) => {
                        self.skip += n;

                        match self.mode.finish_current() {
                            Some(token) => IterTick::Success(token),
                            None => IterTick::Retry,
                        }
                    }
                }
            }

            //  Nothing more can be added. If a tengwa is currently being
            //      constructed, finalize and return it.
            else if let Some(token) = self.mode.finish_current() {
                self.advance_head(0);
                IterTick::Success(token)
            }

            //  Look for a numeric value.
            else if let Some((num, len)) = self.parse_numeral() {
                self.advance_head(len);
                IterTick::Success(Token::Number(num))
            }

            //  Give up and pass the current char through unchanged.
            else {
                IterTick::Success(Token::Char(self.skip_one()))
            }
        } else { // len <= head
            match self.mode.finish_current() {
                Some(token) => IterTick::Success(token),
                None => IterTick::Empty,
            }
        }
    }
}

impl<M: TengwarMode> Iterator for ModeIter<M> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tick() {
                IterTick::Empty => break None,
                IterTick::Retry => continue,
                IterTick::Success(token) => break Some(token),
            }
        }
    }
}
