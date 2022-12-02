use unicode_normalization::UnicodeNormalization;
use crate::{Token, Transcriber};
use super::{ParseAction, TengwarMode};


const fn to_lower(c: char) -> char {
    match c {
        'Ñ' => 'ñ', 'Ð' => 'ð', 'Þ' => 'þ', 'Θ' => 'θ', 'Φ' => 'φ',
        'Ä' => 'ä', 'Ë' => 'ë', 'Ï' => 'ï', 'Ö' => 'ö', 'Ü' => 'ü', 'Ÿ' => 'ÿ',
        'Á' => 'á', 'É' => 'é', 'Í' => 'í', 'Ó' => 'ó', 'Ú' => 'ú', 'Ý' => 'ý',
        'Â' => 'â', 'Ê' => 'ê', 'Î' => 'î', 'Ô' => 'ô', 'Û' => 'û', 'Ŷ' => 'ŷ',
        'Æ' => 'æ', 'Œ' => 'œ',
        'Ɣ' => 'ɣ', 'Ʒ' => 'ʒ',
        _ => c.to_ascii_lowercase(),
    }
}


/// The result of a single "step" of a [`Tokenizer`]. Multiple steps can be
///     performed for each iteration.
#[derive(Clone, Debug)]
enum Step {
    /// The iteration is not complete. Another step should be run immediately.
    Incomplete,
    /// The [`Tokenizer`] is exhausted. The iterator should return [`None`].
    Exhausted,
    /// The iteration is complete. The [`Token`] should now be returned.
    Complete(Token),
}


/// An iterator that yields [`Token`]s from a sequence of [`char`]s, according
///     to the rules defined by the methods of a [`TengwarMode`].
///
/// This is a lower-level construct, and performs only minimal post-processing
///     of the `Token`s, as defined by [`TengwarMode::finalize`]. For a higher
///     level iterator with more powerful rules, consider a [`Transcriber`],
///     which can be created using [`Tokenizer::into_transcriber`].
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
    pub fn new(chars: Vec<char>, mode: M) -> Self {
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
            mode,
            next: None,
        }
    }

    /// Set up a new Tokenizer over UTF-8 text.
    pub fn from_str(s: impl AsRef<str>) -> Self
        where M: Default,
    {
        Self::with_mode(s, M::default())
    }

    /// Set up a new Tokenizer over UTF-8 text with a specific mode instance.
    pub fn with_mode(s: impl AsRef<str>, mode: M) -> Self {
        Self::new(s.as_ref().nfc().collect(), mode)
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

    /// Decrease the width of the slice provided to the [`TengwarMode`]. This is
    ///     done when the Mode implementation fails to report any progress, and
    ///     allows it to try again on smaller data.
    #[inline]
    fn narrow_window(&mut self) { self.size -= 1; }

    /// Add to the number of [`char`]s that will be passed through unaffected in
    ///     subsequent calls to [`Tokenizer::step`].
    #[inline]
    fn skip_count_add(&mut self, n: usize) { self.skip += n; }

    /// Decrement the number of [`char`]s that will be passed unaffected.
    #[inline]
    fn skip_count_dec(&mut self) { self.skip -= 1; }

    /// Perform a single step of parsing. This will result in at most one call
    ///     to [`TengwarMode::process`], and does not guarantee that a [`Token`]
    ///     will be complete by the end. Each `Token` may require several steps.
    fn step(&mut self) -> Step {
        let data: &[char] = &self.lower;
        let mode: &mut M = &mut self.mode;
        let head: usize = self.head;
        let size: usize = self.size;
        let skip: usize = self.skip;
        let len: usize = data.len();

        if len <= head {
            //  The read head is at the end of the data. If a token is currently
            //      being constructed, finish and return it. Otherwise, there is
            //      nothing more to be done, and the tokenizer is now exhausted.
            match mode.finish_current() {
                Some(token) => Step::Complete(token),
                None => Step::Exhausted,
            }
        } else { // head < len
            //  The read head has not reached the end of the data. There is more
            //      work to do.
            if 0 < skip {
                //  The skip counter is currently nonzero. If there is a token
                //      in progress, finish and return it; Otherwise, decrement
                //      the counter and return one `char` directly.
                if let Some(token) = mode.finish_current() {
                    self.advance_head(0);
                    Step::Complete(token)
                } else {
                    self.advance_head(1);
                    self.skip_count_dec();
                    Step::Complete(Token::Char(self.chars[head]))
                }
            }

            //  If the width of the check window has not yet narrowed to zero,
            //      try to parse it according to the Mode implementation.
            else if 0 < size { // skip == 0
                let end: usize = len.min(head + size);
                let chunk: &[char] = &data[head..end];

                match mode.process(chunk) {
                    ParseAction::MatchedNone => {
                        //  No match. Narrow the window and try again.
                        self.narrow_window();
                        Step::Incomplete
                    }
                    ParseAction::MatchedPart(len) => {
                        //  Partial match. Advance the read head and try again.
                        self.advance_head(len);
                        Step::Incomplete
                    }
                    ParseAction::MatchedToken { token, len } => {
                        //  Complete match. Advance the read head and return the
                        //      finished token.
                        self.advance_head(len);
                        Step::Complete(token)
                    }
                    ParseAction::Skip(n) => {
                        //  Skip specified. Increase the skip counter. Then, if
                        //      a token was in progress, return it; Otherwise,
                        //      try again.
                        let finished: Option<Token> = mode.finish_current();
                        self.skip_count_add(n);

                        match finished {
                            Some(token) => Step::Complete(token),
                            None => Step::Incomplete,
                        }
                    }
                    ParseAction::Escape { len_seq, n_skip } => {
                        //  Escape sequence. Advance the read head and increase
                        //      the skip counter, then try again.
                        self.advance_head(len_seq);
                        self.skip_count_add(n_skip);
                        Step::Incomplete
                    }
                }
            }

            //  The chunk width has narrowed to zero. Nothing more can be added.
            //      If a token is currently in progress, finish and return it.
            else if let Some(token) = mode.finish_current() {
                self.advance_head(0);
                Step::Complete(token)
            }

            //  Look for any secondary value in the slice ahead.
            else if let Some((token, len)) = mode.find_secondary(&data[head..]) {
                self.advance_head(len);
                Step::Complete(token)
            }

            //  Give up and pass the current `char` through unchanged.
            else {
                self.advance_head(1);
                Step::Complete(Token::Char(self.chars[head]))
            }
        }
    }

    /// Repeatedly step the iterator until either a new [`Token`] is ready or
    ///     the iterator is exhausted.
    fn step_to_next(&mut self) -> Option<Token> {
        loop {
            match self.step() {
                Step::Incomplete => continue,
                Step::Exhausted => break None,
                Step::Complete(token) => break Some(token),
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
