//! Library for conversion of Latin UTF-8 text into Tengwar, using the unicode
//!     codepoints of the [Free Tengwar Font Project]. Specifically, but not
//!     exclusively, designed with [Tengwar Telcontar] in mind, for the purpose
//!     of use within LaTeX macros.
//!
//! [Free Tengwar Font Project]: http://freetengwar.sourceforge.net/mapping.html
//! [Tengwar Telcontar]: http://freetengwar.sourceforge.net/tengtelc.html
//!
//! # Overview
//!
//! The library is split into two main modules. The [`characters`] module is
//!     primarily concerned with defining the data and datastructures needed to
//!     represent Tengwar. The [`mode`] module, on the other hand, is mainly
//!     concerned with transcription, defining the [`TengwarMode`] trait for the
//!     rules and the [`Tokenizer`](mode::Tokenizer) type for applying them.
//!
//! However, this first level of transcription is usually not enough; Therefore,
//!     the top level of the crate defines the [`TokenIter`] type to perform
//!     additional transformations. This higher-level iterator can be configured
//!     at runtime, and is capable of looking ahead and behind to determine the
//!     context, enabling critical situational behaviors.
//!
//! Three modes are currently provided by default: [`Quenya`] ("Classical"),
//!     [`Beleriand`], and [`Gondor`]. Each mode implements the [`TengwarMode`]
//!     trait.
//!
//! # Examples
//!
//! [`collect`]: Iterator::collect
//!
//! ## `TengwarMode` trait
//!
//! The most direct way to convert text is [`TengwarMode::transcribe`]. This
//!     function accepts any input type that implements `AsRef<str>`, and can
//!     return any type that implements `FromIterator<Token>`; This includes
//!     `Vec<Token>` and [`String`].
//! ```
//! use tengwar::{Quenya, TengwarMode};
//!
//! let text: String = Quenya::transcribe("namárië !");
//! assert_eq!(text, " ");
//! ```
//!
//! ## `ToTengwar` trait
//!
//! With the use of the [`ToTengwar`] helper trait (automatically implemented
//!     for any type implementing `AsRef<str>`), three methods are provided on
//!     the input type directly. The first is [`ToTengwar::transcriber`], which
//!     constructs a [`Transcriber`] for the text, allowing iteration over
//!     [`Token`]s.
//!
//! The `Transcriber` also has [`TranscriberSettings`], holding several public
//!     fields, which can be changed to adjust various aspects of its behavior.
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let mut transcriber = "namárië !".transcriber::<Quenya>();
//! transcriber.settings.alt_a = true; // Use the alternate form of the A-tehta.
//!
//! let text: String = transcriber.collect();
//! assert_eq!(text, " ");
//! ```
//!
//! The second method is [`ToTengwar::to_tengwar`]. This is mostly a convenience
//!     method, which simply calls [`ToTengwar::transcriber`] and immediately
//!     [`collect`]s the Iterator into a [`String`].
//! ```
//! use tengwar::{Quenya, ToTengwar};
//!
//! let text: String = "namárië !".to_tengwar::<Quenya>();
//! assert_eq!(text, " ");
//! ```
//!
//! The third method is [`ToTengwar::to_tengwar_with`], which does the same, but
//!     takes [`TranscriberSettings`] to modify the [`Transcriber`] before it is
//!     collected. This allows settings to be specified once and reused.
//! ```
//! use tengwar::{Quenya, ToTengwar, TranscriberSettings};
//!
//! let mut settings = TranscriberSettings::new();
//! settings.alt_a = true;
//! settings.nuquerna = true;
//!
//! let text: String = "namárië !".to_tengwar_with::<Quenya>(settings);
//! assert_eq!(text, " ");
//!
//! let text: String = "lotsë súva".to_tengwar_with::<Quenya>(settings);
//! assert_eq!(text, " ");
//! ```
//!
//! ## Crate-level function
//!
//! Also available, and likely the easiest to discover via code completion, is
//!     the top-level [`transcribe`] function, which takes an implementor of
//!     [`TengwarMode`] as a generic parameter. This function accepts any input
//!     type that implements [`ToTengwar`], and is a passthrough to the
//!     [`ToTengwar::to_tengwar`] method.
//! ```
//! use tengwar::{Quenya, transcribe};
//!
//! let text: String = transcribe::<Quenya>("namárië !");
//! assert_eq!(text, " ");
//! ```
//!
//! ---
//! # In Detail
//!
//! The core of this library is the [`Token`] enum. A `Token` may hold a simple
//!     [`char`], a [`Glyph`], or a [`Numeral`]. An iterator of `Token`s can be
//!     [`collect`]ed into a [`String`]; This is where the rendering of Tengwar
//!     text truly takes place.
//!
//! The rest of the library is geared around the creation of `Tokens`, usually
//!     by iteration, and modifying them before the final call to `collect`.
//!
//! ## Mode
//!
//! A "Mode" of the Tengwar is essentially an orthography mapping; It correlates
//!     conventions of writing in a primary world alphabet to the conventions of
//!     writing in the Tengwar.
//!
//! For this purpose, the [`TengwarMode`] trait is provided. A type implementing
//!     this trait is expected to perform essentially as a state machine, taking
//!     input in the form of slices of `char`s, and using them to progressively
//!     construct `Token`s.
//!
//! ## Tokenizer
//!
//! The first level of iteration is the [`Tokenizer`](mode::Tokenizer). This
//!     iterator takes UTF-8 text, breaks it down into a [`Vec`] of normalized
//!     Unicode codepoints, and assembles [`Token`]s according to the rules
//!     specified by an implementation of [`TengwarMode`].
//!
//! Short slices of `char`s are passed to the Mode type, which determines
//!     whether to accept them as part of a `Token`. If the `char`s are not
//!     accepted, the slice is narrowed and tried again, until the width reaches
//!     zero; At this point, the Mode type is shown the full remaining data and
//!     asked whether it can get anything at all from it. If it cannot, a `char`
//!     is returned unchanged as a `Token`.
//!
//! When the `Tokenizer` yields a `Token`, the following one is generated. This
//!     allows for one last call to the Mode type, to [`TengwarMode::finalize`],
//!     to modify a `Token` in light of the one that follows it; This is a very
//!     important step, as some modes require that different base characters are
//!     used depending on what follows them.
//!
//! ## TokenIter / Transcriber
//!
//! The second level of iteration is the [`TokenIter`]. This iterator can wrap
//!     any other iterator that produces [`Token`]s, and its purpose is to apply
//!     contextual rules and transformations specified at runtime. This is what
//!     allows the executable transcriber to take CLI options that change rules,
//!     such as the treatment of "long" tehta variants.
//!
//! A `TokenIter` that wraps a [`Tokenizer`](mode::Tokenizer) can also be called
//!     a [`Transcriber`] for simplicity, because it is known that its `Token`s
//!     are being produced directly from text.
//!
//! ## Policy
//!
//! A "Policy" is similar to a Mode, but rather than defining details about
//!     **orthography**, it instead defines details about **typography**. This
//!     includes details such as valid ligatures and placements of *Sa-Rinci*.
//!
//! The [`Policy`](policy::Policy) trait is provided for this purpose, and is
//!     used as a generic parameter for the [`Glyph`] type. Because of this, it
//!     is also a generic parameter for the [`Token`] and [`TokenIter`] types;
//!     The [`Tokenizer`](mode::Tokenizer) type is considered to be out of scope
//!     of the Policy system, and simply yields all of its `Token`s with the
//!     default policy ([`policy::Standard`]).

#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate clap;
#[macro_use]
#[cfg(feature = "serde")]
extern crate serde;

// mod macros;

pub mod characters;
pub mod mode;
pub mod policy;

mod iter;
mod token;

pub use characters::{Glyph, Numeral, VowelStyle};
pub use iter::{TokenIter, Transcriber, TranscriberSettings};
pub use mode::{Beleriand, Gondor, Quenya, TengwarMode};
pub use token::Token;


/// Convert a compatible object (typically text) into the Tengwar.
///
/// This function merely calls a Trait method, but is likely the most readily
///     discoverable part of the library when using code completion tools.
pub fn transcribe<M: TengwarMode + Default>(text: impl ToTengwar) -> String {
    text.to_tengwar::<M>()
}


/// A very small trait serving to implement ergonomic transcription methods
///     directly onto text objects.
pub trait ToTengwar {
    /// Create a [`Transcriber`] to iteratively transcribe this text into the
    ///     Tengwar. The returned iterator will yield [`Token`]s.
    ///
    /// # Example
    /// ```
    /// use tengwar::{Quenya, ToTengwar, VowelStyle};
    ///
    /// const INPUT: &str = "lotsë súva"; // "a flower is sinking"
    ///
    ///
    /// //  Collect directly with default settings.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use Unique Tehtar.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.vowels = VowelStyle::Unique;
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use Nuquernë Tengwar.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.nuquerna = true;
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use Unique Tehtar and Nuquernë Tengwar.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.nuquerna = true;
    /// ts.settings.vowels = VowelStyle::Unique;
    /// assert_eq!(ts.into_string(), " ");
    ///
    ///
    /// //  Use several options.
    /// let mut ts = INPUT.transcriber::<Quenya>();
    /// ts.settings.alt_a = true;
    /// ts.settings.alt_rince = true;
    /// ts.settings.nuquerna = true;
    /// ts.settings.vowels = VowelStyle::Separate;
    /// assert_eq!(ts.into_string(), " ");
    /// ```
    fn transcriber<M: TengwarMode + Default>(&self) -> Transcriber<M>;

    /// Transcribe this object into the Tengwar directly.
    ///
    /// # Example
    /// ```
    /// use tengwar::{Quenya, ToTengwar};
    ///
    /// let text: String = "namárië !".to_tengwar::<Quenya>();
    /// assert_eq!(text, " ");
    /// ```
    fn to_tengwar<M: TengwarMode + Default>(&self) -> String {
        self.transcriber::<M>().into_string()
    }

    /// Transcribe this object into the Tengwar, using [`TranscriberSettings`]
    ///     provided as an argument. This allows the settings to be reused much
    ///     more easily.
    ///
    /// For examples of the available settings, see the documentation of
    ///     [`Self::transcriber`].
    ///
    /// # Example
    /// ```
    /// use tengwar::{Quenya, ToTengwar, TranscriberSettings};
    ///
    /// let mut settings = TranscriberSettings::new();
    /// settings.alt_a = true;
    /// settings.nuquerna = true;
    ///
    /// let text: String = "namárië !".to_tengwar_with::<Quenya>(settings);
    /// assert_eq!(text, " ");
    ///
    /// let text: String = "lotsë súva".to_tengwar_with::<Quenya>(settings);
    /// assert_eq!(text, " ");
    /// ```
    fn to_tengwar_with<M>(&self, settings: TranscriberSettings) -> String
        where M: TengwarMode + Default
    {
        self.transcriber::<M>().with_settings(settings).into_string()
    }
}

impl<S: AsRef<str>> ToTengwar for S {
    fn transcriber<M: TengwarMode + Default>(&self) -> Transcriber<M> {
        mode::Tokenizer::from_str(self).into_transcriber()
    }
}
