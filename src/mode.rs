//! This module defines the interface, and default implementations, for "modes"
//!     of the Tengwar: High-level rules for text representations.
//!
//! The core of the module is the [`TengwarMode`] trait, which defines the
//!     common functionality. The [`Tokenizer`] iterator takes input text and
//!     uses the rules of a mode to construct [`Token`]s.
//!
//! There are three modes currently implemented by default: [`Quenya`],
//!     [`Gondor`], and [`Beleriand`].

pub mod beleriand;
pub mod custom;
// pub mod general;
pub mod gondor;
pub mod quenya;

mod iter;
mod tests;

pub use beleriand::Beleriand;
#[cfg(feature = "mode-custom")]
pub use custom::CustomMode;
// pub use general::General;
pub use gondor::Gondor;
pub use quenya::Quenya;
pub use iter::Tokenizer;

use crate::{Numeral, Token, Transcriber};


/// The result of a call to [`TengwarMode::process`]. This specifies the next
///     action that will be taken by a [`Tokenizer`].
#[derive(Clone, Debug)]
pub enum ParseAction {
    /// Nothing could be done with the input.
    MatchedNone,
    /// A portion of the input was processed successfully, but more is needed.
    MatchedPart(usize),
    /// A complete [`Token`] has been processed.
    MatchedToken {
        /// The completed [`Token`].
        token: Token,
        /// The number of [`char`]s that were processed during the final step of
        ///     parsing this Token.
        len: usize,
    },
    /// A portion of the input has been determined to be irrelevant to the
    ///     processing. The contained number of [`char`]s should be passed
    ///     through unchanged.
    Skip(usize),
    /// An escape sequence has been found. The next [`char`] will be ignored,
    ///     and the one following will be passed through unchanged.
    Escape {
        /// The number of [`char`]s in the escape sequence itself. These will be
        ///     ignored, and will not be present in the output in any way.
        len_seq: usize,
        /// The number of [`char`]s affected by the escape sequence. These will
        ///     be passed through to the output without being processed.
        n_skip: usize,
    },
}

impl ParseAction {
    /// The behavior of a simple backslash escape sequence.
    pub const ESC_BACKSLASH: Self = Self::Escape { len_seq: 1, n_skip: 1 };
}


/// This trait defines a "mode" of transcription of text into the Tengwar. It
///     implements methods that receive slices of [`char`]s and progressively
///     construct [`Token`]s held in internal state.
#[allow(unused_variables)]
pub trait TengwarMode {
    /// This is the maximum size for a "chunk" of [`char`]s passed to
    ///     [`Self::process`]. It is also the maximum number of passes that will
    ///     be attempted before skipping a `char` and moving on.
    const MAX_CHUNK: usize = 3;

    /// Set up a [`Transcriber`] over the characters of an input string, and
    ///     immediately collect it into the target type.
    fn transcribe<T: FromIterator<Token>>(input: impl AsRef<str>) -> T
        where Self: Default,
    {
        Self::default_transcriber(input).collect()
    }

    /// Set up a [`Transcriber`] over the characters of an input string, using
    ///     the default initial state of this mode.
    fn default_transcriber(input: impl AsRef<str>) -> Transcriber<Self>
        where Self: Default,
    {
        Self::default().into_transcriber(input)
    }

    /// Set up a [`Transcriber`] over the characters of an input string.
    fn into_transcriber(self, input: impl AsRef<str>) -> Transcriber<Self>
        where Self: Sized,
    {
        Tokenizer::<Self>::with_mode(input, self).into_transcriber()
    }

    /// Perform any last-minute modifications to a [`Token`] that may be needed
    ///     upon finding out what the following `Token` will be. By default,
    ///     this method is a no-op.
    fn finalize(&self, token: &mut Token, next: Option<&Token>) {}

    /// Try to parse a slice of characters into an "index" of a sequence. This
    ///     special case of a numeral is intended for use in enumerated lists.
    fn find_index(&mut self, slice: &[char]) -> Option<(char, usize)> {
        crate::characters::numeral::find_index(slice)
    }

    /// Try to parse a slice of characters into a [`Numeral`]. If successful,
    ///     returns the `Numeral` alongside the number of [`char`]s that were
    ///     processed in order to find it.
    ///
    /// The input slice is NOT bounded by [`MAX_CHUNK`], and extends to the end
    ///     of the data.
    ///
    /// [`MAX_CHUNK`]: Self::MAX_CHUNK
    fn find_numeral(&mut self, slice: &[char]) -> Option<(Numeral, usize)> {
        Numeral::parse(slice)
    }

    /// If there is a [`Token`] currently under construction, return it and
    ///     clear it from the internal state, preparing to begin a new one.
    ///
    /// Ideally, this method should be cheap to call, because it will be called
    ///     whenever [`next`] is called on a [`Tokenizer`] which has reached the
    ///     end of its data.
    ///
    /// [`next`]: Iterator::next
    fn finish_current(&mut self) -> Option<Token>;

    /// Process a slice of [`char`]s, and return a [`ParseAction`] indicating
    ///     the new state of the conversion in progress.
    ///
    /// Assuming this method is called by a [`Tokenizer`], the input slice will
    ///     be no more than [`MAX_CHUNK`] in length, but it may be shorter.
    ///
    /// [`MAX_CHUNK`]: Self::MAX_CHUNK
    fn process(&mut self, chunk: &[char]) -> ParseAction;
}
