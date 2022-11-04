use crate::{characters::punctuation, Token, Transcriber};
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
    /// The original data, with case intact.
    chars: Vec<char>,
    /// Data vec, converted to ASCII lowercase.
    lower: Vec<char>,

    /// Current position within the data vec.
    head: usize,
    /// Width of the current working window.
    size: usize,
    /// Number of [`char`]s that will be passed through unchanged, starting at
    ///     `data[head]`.
    skip: usize,

    /// The operating Mode, which determines the actual transcription rules.
    pub mode: M,
    next: Option<Token>,
}

/// Public functionality.
impl<M: TengwarMode> ModeIter<M> {
    pub fn new(chars: Vec<char>) -> Self {
        let size: usize = chars.len().min(M::MAX_CHUNK);
        let mut lower = chars.clone();

        for char in &mut lower {
            char.make_ascii_lowercase();
        }

        Self {
            chars,
            lower,
            head: 0,
            size,
            skip: 0,
            mode: M::default(),
            next: None,
        }
    }

    pub fn from_str(s: impl AsRef<str>) -> Self {
        Self::new(s.as_ref().chars().collect())
    }

    pub fn current(&self) -> &char {
        &self.chars[self.head]
    }

    pub fn into_token_iter(self) -> Transcriber<Self> {
        self.into()
    }

    pub fn window(&self) -> &[char] {
        let end: usize = self.chars.len().min(self.head + self.size);
        &self.chars[self.head..end]
    }

    pub fn window_lower(&self) -> &[char] {
        let end: usize = self.lower.len().min(self.head + self.size);
        &self.lower[self.head..end]
    }
}

/// Internal functionality.
impl<M: TengwarMode> ModeIter<M> {
    fn advance_head(&mut self, n: usize) {
        self.head += n;
        self.size = self.chars.len().min(M::MAX_CHUNK);
    }

    fn skip_one(&mut self) -> char {
        let here: char = *self.current();
        self.advance_head(1);
        here
    }

    fn step(&mut self) -> IterStep {
        let &mut Self {
            chars: _,
            lower: ref data,
            head, size, skip,
            ref mut mode,
            next: _
        } = self;
        let len: usize = data.len();

        if head < len {
            //  Obey the "skip" counter above all else.
            if 0 < skip {
                if let Some(token) = mode.finish_current() {
                    self.advance_head(0);
                    IterStep::Success(token)
                } else {
                    self.skip -= 1;
                    IterStep::Success(Token::Char(self.skip_one()))
                }
            }

            //  Delegate matching to the Mode implementation.
            else if 0 < size { // skip == 0
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

                        match mode.finish_current() {
                            Some(token) => IterStep::Success(token),
                            None => IterStep::Again,
                        }
                    }
                    ParseAction::Escape => {
                        self.skip += 1;
                        self.advance_head(1);
                        IterStep::Again
                    }
                }
            }

            //  Nothing more can be added. If a tengwa is currently being
            //      constructed, finalize and return it.
            else if let Some(token) = mode.finish_current() {
                self.advance_head(0);
                IterStep::Success(token)
            }

            //  Look for a numeric value.
            else if let Some((num, len)) = mode.find_numeral(&data[head..]) {
                self.advance_head(len);
                IterStep::Success(Token::Number(num))
            }

            //  Check for punctuation.
            else if let Some(punct) = punctuation(data[head]) {
                self.advance_head(1);
                IterStep::Success(Token::Char(punct))
            }

            //  Give up and pass the current `char` through unchanged.
            else {
                IterStep::Success(Token::Char(self.skip_one()))
            }
        } else { // len <= head
            //  The read head is at the end of the data. If a tengwa is
            //      currently being constructed, finalize and return it.
            match mode.finish_current() {
                Some(token) => IterStep::Success(token),
                None => IterStep::Empty,
            }
        }
    }

    /// Repeatedly step the iterator until either a new [`Token`] is ready or
    ///     the iterator is exhausted.
    fn step_to_next(&mut self) -> Option<Token> {
        loop {
            match self.step() {
                IterStep::Again => continue,
                IterStep::Empty => break None,
                IterStep::Success(token) => break Some(token),
            }
        }
    }
}

impl<M: TengwarMode> Iterator for ModeIter<M> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: Token = match self.next.take() {
            Some(stored) => stored,
            None => self.step_to_next()?,
        };

        self.next = self.step_to_next();
        self.mode.finalize(&mut token, self.next.as_ref());
        Some(token)
    }
}
