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
pub use iter::ModeIter;

use crate::{Numeral, Token};


/// The result of a call to [`TengwarMode::process`]. This specifies the next
///     action that will be taken by a [`ModeIter`].
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
    ///     processing. The contained number of [`char`]s should be passed
    ///     through unchanged.
    Skip(usize),
    /// An escape sequence has been found. The next [`char`] will be ignored,
    ///     and the one following will be passed through unchanged.
    Escape,
}


/// This trait defines a "mode" of transcription of text into the Tengwar. It
///     implements methods that receive slices of [`char`]s and progressively
///     construct [`Token`]s held in internal state.
#[allow(unused_variables)]
pub trait TengwarMode: Default + Sized {
    /// This is the maximum size for a "chunk" of [`char`]s passed to
    ///     [`Self::process`]. It is also the maximum number of passes that will
    ///     be attempted before skipping a `char` and moving on.
    const MAX_CHUNK: usize = 3;

    /// Construct a [`ModeIter`] of this mode to transliterate the input data.
    fn mode_iter(chars: Vec<char>) -> ModeIter<Self> {
        ModeIter::new(chars)
    }

    /// Set up a [`TokenIter`] over the characters of an input string, and
    ///     immediately collect it into the target type.
    ///
    /// [`TokenIter`]: crate::TokenIter
    fn transcribe<T: FromIterator<Token>>(input: impl AsRef<str>) -> T {
        ModeIter::<Self>::from_str(input).into_token_iter().collect()
    }

    /// Perform any last-minute modifications to a [`Token`] that may be needed
    ///     upon finding out what the next `Token` will be. By default, this
    ///     method is a no-op.
    fn finalize(&self, token: &mut Token, next: Option<&Token>) {}

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
    ///     clear it from the internal state, preparing to begin a new tengwa.
    ///
    /// Ideally, this method should be cheap to call, because it will be called
    ///     whenever [`next`] is called on a [`ModeIter`] which has reached the
    ///     end of its data.
    ///
    /// [`next`]: Iterator::next
    fn finish_current(&mut self) -> Option<Token>;

    /// Process a slice of [`char`]s, and return a [`ParseAction`] indicating
    ///     the new state of the conversion in progress.
    ///
    /// Assuming this method is called by a [`ModeIter`], the input slice will
    ///     be no more than [`MAX_CHUNK`] in length, but it may be shorter.
    ///
    /// [`MAX_CHUNK`]: Self::MAX_CHUNK
    fn process(&mut self, chunk: &[char]) -> ParseAction;
}


#[test]
fn test_iter() {
    fn convert(text: impl AsRef<str>) -> String {
        quenya::Quenya::transcribe(text)
    }

    eprintln!("{}", convert("yéni 144"));
}
