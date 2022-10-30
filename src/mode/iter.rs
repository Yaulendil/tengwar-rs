use crate::Token;
use super::{ParseAction, TengwarMode};


#[derive(Clone, Debug)]
enum IterTick {
    Empty,
    Retry,
    Success(Token),
}


#[derive(Debug)]
pub struct ModeIter<M: TengwarMode> {
    data: Vec<char>,
    head: usize,
    size: usize,
    skip: usize,
    mode: M,
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

    fn skip_one(&mut self) -> char {
        let here: char = self.data[self.head];
        self.advance_head(1);
        here
    }

    fn tick(&mut self) -> IterTick {
        let len: usize = self.data.len();

        if self.head < len {
            if 0 < self.skip {
                self.skip -= 1;
                IterTick::Success(Token::Char(self.skip_one()))
            } else if 0 < self.size { // skip == 0
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
                        IterTick::Retry
                    }
                }
            } else { // skip == 0 && size == 0
                IterTick::Success(Token::Char(self.skip_one()))
            }
        } else {
            IterTick::Empty
        }
    }
}

impl<M: TengwarMode> Iterator for ModeIter<M> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match dbg!(self.tick()) {
                IterTick::Empty => break None,
                IterTick::Retry => continue,
                IterTick::Success(token) => break Some(token),
            }
        }
    }
}
