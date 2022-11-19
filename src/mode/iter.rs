use unicode_normalization::UnicodeNormalization;
use crate::{characters::punctuation, Token, Transcriber};
use super::{ParseAction, TengwarMode};


const fn to_lower(c: char) -> char {
    match c {
        'Ñ' => 'ñ', 'Ð' => 'ð', 'Þ' => 'þ', 'Θ' => 'θ', 'Φ' => 'φ',
        'Ä' => 'ä', 'Ë' => 'ë', 'Ï' => 'ï', 'Ö' => 'ö', 'Ü' => 'ü', 'Ÿ' => 'ÿ',
        'Á' => 'á', 'É' => 'é', 'Í' => 'í', 'Ó' => 'ó', 'Ú' => 'ú', 'Ý' => 'ý',
        'Â' => 'â', 'Ê' => 'ê', 'Î' => 'î', 'Ô' => 'ô', 'Û' => 'û', 'Ŷ' => 'ŷ',
        _ => c.to_ascii_lowercase(),
    }
}


/// The result of a single "step" of a [`Tokenizer`]. Multiple steps can be
///     performed for each iteration.
#[derive(Clone, Debug)]
enum IterStep {
    /// The iteration is not complete. Another step should be run immediately.
    Again,
    /// The [`Tokenizer`] is exhausted. The iterator can safely return [`None`].
    Empty,
    /// The iteration is complete. The [`Token`] should now be returned.
    Success(Token),
}


/// An iterator that yields [`Token`]s from a sequence of [`char`]s, according
///     to the rules defined by the methods of a [`TengwarMode`].
///
/// This is a lower-level construct, and performs only minimal post-processing
///     of the `Token`s, as defined by [`TengwarMode::finalize`]. For a higher
///     level iterator with more powerful rules, see [`crate::Transcriber`].
#[derive(Debug)]
pub struct Tokenizer<M: TengwarMode> {
    /// The original data, with case intact.
    chars: Vec<char>,
    /// Data vec, converted to lowercase for processing.
    lower: Vec<char>,

    /// Current position within the data vec.
    head: usize,
    /// Width of the current working window.
    size: usize,
    /// Number of [`char`]s that will be passed through unchanged, starting at
    ///     `chars[head]`.
    skip: usize,

    /// The operating Mode, which determines the actual tokenization rules.
    pub mode: M,
    next: Option<Token>,
}

/// Public functionality.
impl<M: TengwarMode> Tokenizer<M> {
    /// Set up a new Tokenizer over a sequence of [`char`]s.
    pub fn new(chars: Vec<char>) -> Self {
        let size: usize = chars.len().min(M::MAX_CHUNK);
        let mut lower = chars.clone();

        for char in &mut lower {
            *char = to_lower(*char);
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

    /// Set up a new Tokenizer over UTF-8 text.
    pub fn from_str(s: impl AsRef<str>) -> Self {
        Self::new(s.as_ref().nfc().collect())
    }

    /// Wrap this [`Tokenizer`] in a [`Transcriber`] that can apply higher-level
    ///     rules.
    pub fn into_transcriber(self) -> Transcriber<M> { self.into() }

    /// Return the slice of original [`char`]s, corresponding to the ones that
    ///     will be processed in the next step.
    pub fn window(&self) -> &[char] {
        let end: usize = self.chars.len().min(self.head + self.size);
        &self.chars[self.head..end]
    }

    /// Return the slice of lowercase [`char`]s that will be processed in the
    ///     next step.
    pub fn window_lower(&self) -> &[char] {
        let end: usize = self.lower.len().min(self.head + self.size);
        &self.lower[self.head..end]
    }
}

/// Internal functionality.
impl<M: TengwarMode> Tokenizer<M> {
    /// Move the read head forward and reset the window width.
    fn advance_head(&mut self, n: usize) {
        self.head += n;
        self.size = self.chars.len().min(M::MAX_CHUNK);
    }

    /// Pass along one [`char`], exactly as it is in the input.
    fn skip_one(&mut self) -> char {
        let here: char = self.chars[self.head];
        self.advance_head(1);
        here
    }

    /// Perform a single step of parsing. This will result in at most one call
    ///     to [`TengwarMode::process`], and does not guarantee that a [`Token`]
    ///     will be complete by the end. Each `Token` may require several steps.
    fn step(&mut self) -> IterStep {
        let &mut Self {
            chars: _,
            lower: ref data,
            head, size, skip,
            ref mut mode,
            next: _,
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

            //  Nothing more can be added. If a token is currently being
            //      constructed, finish and return it.
            else if let Some(token) = mode.finish_current() {
                self.advance_head(0);
                IterStep::Success(token)
            }

            //  Look for a sequence index.
            else if let Some((char, len)) = mode.find_index(&data[head..]) {
                self.advance_head(len);
                IterStep::Success(Token::Char(char))
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
            //  The read head is at the end of the data. If a token is
            //      currently being constructed, finish and return it.
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

impl<M: TengwarMode> Iterator for Tokenizer<M> {
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
