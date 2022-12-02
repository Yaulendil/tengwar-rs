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
//!     the top-level [`crate::transcribe`] function, which takes an implementor
//!     of [`TengwarMode`] as a generic parameter. This function accepts any
//!     input type that implements [`ToTengwar`], and is a passthrough to the
//!     [`ToTengwar::to_tengwar`] method.
//! ```
//! use tengwar::{Quenya, transcribe};
//!
//! let text: String = transcribe::<Quenya>("namárië !");
//! assert_eq!(text, " ");
//! ```

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
