use std::iter::Peekable;
use crate::{
    characters::*,
    mode::Tokenizer,
    policy::{Policy, Standard},
    token::Token,
};


/// An iterator over a sequence of [`Token`]s which applies various rules. This
///     is the top level construct of the transcription process.
///
/// This type is a special case of a [`TokenIter`], where the internal iterator
///     is a [`Tokenizer`].
pub type Transcriber<M, P = Standard> = TokenIter<Tokenizer<M>, Standard, P>;


/// An iterator over a sequence of [`Token`]s which applies various rules. This
///     is the top level construct of the transcription process.
///
/// This iterator is intended to work with a [`Tokenizer`], but is able to work
///     with any type that iterates over [`Token`]s. This allows the tokens to
///     be filtered or modified after being parsed, but before the surrounding
///     context is analyzed. This ability may be critical to perform changes
///     that would affect the context.
pub struct TokenIter<I: Iterator<Item=Token<P>>, P: Policy, Q: Policy = P> {
    inner: Peekable<I>,
    last: Option<Token<Q>>,
    pub settings: TranscriberSettings,
}

impl<I: Iterator<Item=Token<P>>, P: Policy> TokenIter<I, P, P> {
    /// Construct a TokenIter around an arbitrary Iterator of [`Token`]s.
    pub fn new(iter: I) -> Self {
        Self {
            inner: iter.peekable(),
            last: None,
            settings: Default::default(),
        }
    }
}

impl<I: Iterator<Item=Token<P>>, P: Policy, Q: Policy> TokenIter<I, P, Q> {
    /// Collect all [`Token`]s into a [`String`].
    pub fn into_string(self) -> String { self.collect() }

    /// Return a reference to the previous Token.
    pub fn last(&self) -> Option<&Token<Q>> { self.last.as_ref() }

    /// Return a reference to the next Token, without advancing the Iterator.
    pub fn peek(&mut self) -> Option<&Token<P>> { self.inner.peek() }

    /// Change the [`Policy`] used for the [`Glyph`]s produced by this iterator.
    pub fn set_policy<R: Policy>(self) -> TokenIter<I, P, R> {
        TokenIter {
            inner: self.inner,
            last: self.last.map(Token::change_policy),
            settings: self.settings,
        }
    }

    /// Change the transcription behavior settings.
    pub const fn with_settings(mut self, new: TranscriberSettings) -> Self {
        self.settings = new;
        self
    }
}

impl<I, P> From<I> for TokenIter<I::IntoIter, P, P> where
    I: IntoIterator<Item=Token<P>>,
    P: Policy,
{
    fn from(iter: I) -> Self { Self::new(iter.into_iter()) }
}

impl<I, P, Q> Iterator for TokenIter<I, P, Q> where
    I: Iterator<Item=Token<P>>,
    P: Policy,
    Q: Policy,
{
    type Item = Token<Q>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token: Token<Q> = self.inner.next()?.change_policy::<Q>();

        if let Token::Glyph(glyph) = &mut token {
            glyph.ligate_zwj = self.settings.ligate_zwj;
            glyph.nuquerna = self.settings.nuquerna;
            glyph.vowels = self.settings.vowels;

            if self.settings.dot_plain && !glyph.carries_tehta() {
                glyph.dot_under = true;
            }

            if self.settings.elide_a {
                glyph.elide_a();
            }

            if self.settings.alt_a {
                glyph.set_alt_a();
            }

            match self.inner.peek() {
                Some(Token::Glyph(next)) => {
                    glyph.rince_final = false;
                    glyph.ligate_short = self.settings.ligate_short
                        // && glyph.is_short_carrier()
                        && next.telco_ligates();
                }
                _ => {
                    glyph.rince_final = self.settings.alt_rince;
                    glyph.ligate_short = false;
                }
            }
        }

        self.last = Some(token);
        self.last
    }
}


/// Behavior settings to be used by a [`TokenIter`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TranscriberSettings {
    /// If this is `true`, the [A-tehta](TEHTA_A) will be replaced with its
    ///     [alternate form](TEHTA_YANTA).
    pub alt_a: bool,

    /// If this is `true`, [Sa-Rinci](SA_RINCE) at the ends of words will use
    ///     the [alternate form](SA_RINCE_FINAL) where appropriate.
    pub alt_rince: bool,

    /// If this is `true`, any tengwa that does not carry a tehta will be marked
    ///     with a dot.
    pub dot_plain: bool,

    /// If this is `true`, the [A-tehta](TEHTA_A) will not be used.
    pub elide_a: bool,

    /// If this is `true`, the [short carrier](CARRIER_SHORT) will be replaced
    ///     by its [ligating variant](CARRIER_SHORT_LIG) where appropriate.
    pub ligate_short: bool,

    /// If this is nonzero, [zero-width joiners](ZWJ) will be placed between
    ///     glyphs to form font ligatures where appropriate.
    pub ligate_zwj: u8,

    /// If this is `true`, the characters [Silmë](TENGWA_SILME) and
    ///     [Essë](TENGWA_ESSE) will use their inverted Nuquernë variants when
    ///     holding a tehta.
    pub nuquerna: bool,

    /// This defines the treatment of "long" vowels.
    pub vowels: VowelStyle,
}

impl TranscriberSettings {
    /// Define new settings for a [`TokenIter`].
    pub const fn new() -> Self {
        Self {
            alt_a: false,
            alt_rince: false,
            dot_plain: false,
            elide_a: false,
            ligate_short: false,
            ligate_zwj: 0,
            nuquerna: false,
            vowels: VowelStyle::DEFAULT,
        }
    }
}

impl Default for TranscriberSettings {
    fn default() -> Self { Self::new() }
}
