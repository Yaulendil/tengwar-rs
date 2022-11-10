use std::fmt::{Display, Formatter, Write};
use super::*;


enum TehtaChar {
    OnAraAfter(char),
    OnAraBefore(char),
    OnTengwaOnce(char),
    OnTengwaTwice(char),
}


/// A single base tengwa, and all of its modifications. This includes the tehta
///     marking, flags for additional diacritics, flags for consonant and vowel
///     length, and information on vowel and ligature behavior.
#[derive(Clone, Copy, Debug, Default)]
pub struct Glyph {
    /// A base character.
    pub base: Option<char>,
    /// The primary diacritical marking over the base character.
    pub tehta: Option<Tehta>,
    /// Indicates whether the [`Tehta`] should use its alternate "long" form.
    pub tehta_alt: bool,
    /// Indicates whether a tehta with an extended carrier should be printed
    ///     before the glyph.
    pub tehta_first: bool,

    /// The pattern of behavior followed by the tehta, if there is one.
    pub vowels: VowelStyle,

    /// Indicates whether the glyph has a sa-rincë attached.
    pub rince: bool,
    /// Indicates whether the glyph may use a more ornate rincë. This will have
    ///     no effect if `rince` is not `true`.
    pub rince_final: bool,

    /// A nasalized consonant is typically represented by an overbar.
    pub nasal: bool,
    /// A labialized consonant is represented by an additional diacritic.
    pub labial: bool,
    /// A palatalized vowel is represented by an additional diacritic.
    pub palatal: bool,
    /// Try to use a "nuquerna" variant of the base character.
    pub nuquerna: bool,
    /// A lengthened consonant is typically represented by an underbar.
    pub long_cons: bool,

    /// Indicates whether a dot should be placed inside the base character. This
    ///     is occasionally used, when vowel tehtar may be elided, to indicate
    ///     the explicit lack of a vowel.
    pub dot_inner: bool,

    /// Indicates whether a dot should be placed below the base character. This
    ///     is used in many English modes to indicate a "silent" E.
    pub dot_under: bool,

    /// Indicates that this glyph should use the [ligating short carrier], if it
    ///     is applicable.
    ///
    /// [ligating short carrier]: CARRIER_SHORT_LIG
    pub ligate_short: bool,

    /// Indicates whether this glyph should try to use [`ZWJ`] ligation if it
    ///     needs to output a separate carrier. This does NOT cause the glyph to
    ///     try to ligate with the NEXT glyph.
    pub ligate_zwj: bool,
}

impl Glyph {
    /// Define a new empty glyph.
    pub const fn new() -> Self {
        Self {
            base: None,
            tehta: None,
            tehta_alt: false,
            tehta_first: false,
            vowels: VowelStyle::DEFAULT,

            rince: false,
            rince_final: false,

            nasal: false,
            labial: false,
            palatal: false,
            nuquerna: false,
            long_cons: false,

            dot_inner: false,
            dot_under: false,
            ligate_short: false,
            ligate_zwj: false,
        }
    }

    /// Define a glyph with both a base [`char`] and a [`Tehta`].
    pub const fn new_both(base: char, tehta: Tehta) -> Self {
        Self { base: Some(base), tehta: Some(tehta), ..Self::new() }
    }

    /// Define a glyph with only a base [`char`].
    pub const fn new_base(base: char) -> Self {
        Self { base: Some(base), ..Self::new() }
    }

    /// Define a glyph with only a [`Tehta`].
    pub const fn new_tehta(tehta: Tehta) -> Self {
        Self { tehta: Some(tehta), ..Self::new() }
    }

    /// Define a glyph with only an alternate [`Tehta`].
    pub const fn new_tehta_alt(tehta: Tehta) -> Self {
        Self { tehta: Some(tehta), tehta_alt: true, ..Self::new() }
    }

    /// Define a glyph with only a [`Tehta`]. It may be marked as Alternate.
    pub const fn new_vowel(tehta: Tehta, alt: bool) -> Self {
        Self { tehta: Some(tehta), tehta_alt: alt, ..Self::new() }
    }

    pub const fn with_tengwa(mut self, tengwa: char) -> Self {
        self.base = Some(tengwa);
        self
    }

    pub const fn with_tehta(mut self, tehta: Tehta) -> Self {
        self.tehta = Some(tehta);
        self
    }

    pub const fn with_tehta_alt(mut self) -> Self {
        self.tehta_alt = true;
        self
    }

    /// Mark this glyph as being underlined.
    pub const fn with_underline(mut self) -> Self {
        self.long_cons = true;
        self
    }

    /// Mark this glyph as being labialized. It will be rendered with a wavy
    ///     overbar.
    pub const fn with_labial(mut self) -> Self {
        self.labial = true;
        self
    }

    /// Mark this glyph as being nasalized. It will be rendered overlined.
    pub const fn with_nasal(mut self) -> Self {
        self.nasal = true;
        self
    }

    /// Mark this glyph as being palatalized. It will be rendered with a pair of
    ///     dots below it.
    pub const fn with_palatal(mut self) -> Self {
        self.palatal = true;
        self
    }

    /// Mark this glyph as being followed by a sibilant. It may be rendered with
    ///     a flourish.
    pub const fn with_rince(mut self) -> Self {
        self.rince = true;
        self
    }

    /// Update this glyph with the consonant attributes of another glyph.
    pub fn integrate_consonant(&mut self, other: Self) {
        self.base = other.base;
        self.rince = other.rince;
        self.nasal = other.nasal;
        self.labial = other.labial;
        self.palatal = other.palatal;
        self.long_cons = other.long_cons;
    }

    /// Update this glyph with the vowel attributes of another glyph.
    pub fn integrate_vowel(&mut self, other: Self) {
        self.tehta = other.tehta;
        self.tehta_alt = other.tehta_alt;
    }

    /// If the base [`char`] matches a specific value, change it to another.
    pub fn replace_base(&mut self, old: char, new: char) -> bool {
        if self.base == Some(old) {
            self.base = Some(new);
            true
        } else {
            false
        }
    }

    /// If the [`Tehta`] matches a specific value, change it to another.
    pub fn replace_tehta(&mut self, old: Tehta, new: Tehta) -> bool {
        if self.tehta == Some(old) {
            self.tehta = Some(new);
            true
        } else {
            false
        }
    }

    /// Switch the [A-tehta](TEHTA_A) to its [alternate form](TEHTA_YANTA).
    pub fn set_alt_a(&mut self) -> bool {
        self.replace_tehta(TEHTA_A, TEHTA_YANTA)
    }
}

impl Glyph {
    /// Determine the base character to be used for this glyph. If one is not
    ///     set, an appropriate "carrier" mark will be returned instead.
    pub const fn base(&self) -> char {
        match self {
            &Glyph {
                base: Some(base),
                tehta: Some(tehta),
                tehta_alt,
                nuquerna: true,
                ..
            } if nuquerna_valid(base) && !(tehta_alt && tehta.needs_ara()) => {
                nuquerna(base)
            }

            &Glyph { base: Some(base), .. } => base,
            &Glyph { tehta_alt, ligate_short, .. } => {
                if tehta_alt {
                    CARRIER_LONG
                } else if ligate_short {
                    CARRIER_SHORT_LIG
                } else {
                    CARRIER_SHORT
                }
            }
        }
    }

    /// Determine whether a rincë may be added to this glyph. Returns `false` if
    ///     a rincë is already set.
    pub const fn can_take_rince(&self) -> bool {
        !self.rince && rince_valid(self.base())
    }

    /// Determine whether the base character has a nuquerna variant, but is set
    ///     to not use it.
    pub const fn ignoring_nuquerna(&self) -> bool {
        match self.base {
            Some(base) if !self.nuquerna => nuquerna_valid(base),
            _ => false,
        }
    }

    /// Determine whether this glyph will use [Telco](TENGWA_TELCO) as its base.
    pub const fn is_short_carrier(&self) -> bool {
        match self {
            Self { base: None, tehta: None, .. } => true,
            Self { base: None, tehta_alt: false, .. } => true,
            Self { .. } => false,
        }
    }

    /// Determine whether the base [`char`] of this glyph is permitted to ligate
    ///     with another glyph using a zero-width joiner.
    pub const fn ligates_with(&self, other: &Self) -> bool {
        ligature_valid(self, other)
    }

    /// Determine whether the base [`char`] of this glyph is permitted to ligate
    ///     with [Ára](TENGWA_ARA) using a zero-width joiner.
    pub const fn ligates_with_ara(&self) -> bool {
        ligates_with_ara(self.base())
    }

    /// Determine whether [Telco](TENGWA_TELCO) is permitted to ligate with the
    ///     base [`char`] of this glyph using a zero-width joiner.
    pub const fn telco_ligates(&self) -> bool {
        telco_ligates_with(self.base())
    }
}

impl Glyph {
    /// Resolve the position and identity of the tehta.
    const fn tehta_char(&self) -> Option<TehtaChar> {
        let Some(tehta) = self.tehta else {
            //  If there is no tehta, there is nothing to use for it.
            return None;
        };
        let Some(tengwa) = self.base else {
            //  If there is no tengwa, the base will be a carrier already.
            return Some(TehtaChar::OnTengwaOnce(tehta.base));
        };

        let needs_ara;
        let is_double;
        let char;

        match (self.vowels, tehta) {
            (VowelStyle::Doubled, Tehta { base, can_double: true, .. }) => {
                needs_ara = false;
                is_double = self.tehta_alt;
                char = base;
            }
            (VowelStyle::Unique, Tehta { base, alternate: Some(alt), .. }) => {
                needs_ara = false;
                is_double = false;
                char = if self.tehta_alt { alt } else { base };
            }
            (_, Tehta { base, .. }) => {
                needs_ara = self.tehta_alt;
                is_double = false;
                char = base;
            }
        }

        //  If the base tengwa has a Nuquerna variant, but it is not going to be
        //      used, the standard form cannot hold a double or alternate tehta.
        let nuq_ignored = !self.nuquerna && nuquerna_valid(tengwa);
        let cannot_hold = self.tehta_alt && nuq_ignored;

        if cannot_hold || needs_ara {
            if self.tehta_first {
                Some(TehtaChar::OnAraBefore(char))
            } else {
                Some(TehtaChar::OnAraAfter(char))
            }
        } else {
            if is_double {
                Some(TehtaChar::OnTengwaTwice(char))
            } else {
                Some(TehtaChar::OnTengwaOnce(char))
            }
        }
    }

    fn write_mods(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.nasal { f.write_char(MOD_NASAL)?; }
        if self.long_cons { f.write_char(MOD_LONG_CONS)?; }
        if self.labial { f.write_char(MOD_LABIAL)?; }
        if self.palatal { f.write_char(MOD_PALATAL)?; }
        if self.dot_inner { f.write_char(DC_INNER_DOT_1)?; }
        if self.dot_under { f.write_char(DC_UNDER_DOT_1)?; }
        Ok(())
    }

    fn write_rince(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.rince {
            f.write_char(rince(self.base(), self.rince_final))
        } else {
            Ok(())
        }
    }

    fn write_rince_nonfinal(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.rince {
            f.write_char(SA_RINCE)
        } else {
            Ok(())
        }
    }
}

impl Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base: char = self.base();

        match self.tehta_char() {
            Some(TehtaChar::OnAraAfter(tehta)) => {
                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince_nonfinal(f)?;

                if self.ligate_zwj && ligates_with_ara(base) {
                    f.write_char(ZWJ)?;
                }

                f.write_char(CARRIER_LONG)?;
                f.write_char(tehta)?;
            }
            Some(TehtaChar::OnAraBefore(tehta)) => {
                f.write_char(CARRIER_LONG)?;
                f.write_char(tehta)?;

                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince(f)?;
            }
            Some(TehtaChar::OnTengwaOnce(tehta)) => {
                f.write_char(base)?;
                self.write_mods(f)?;

                f.write_char(tehta)?;
                self.write_rince(f)?;
                //  TODO: If there will be a nonfinal rincë in this position,
                //      print it BEFORE the tehta. Discovered an issue with the
                //      base rincë, when placed after the tehta, not combining
                //      properly with the unique long forms.
            }
            Some(TehtaChar::OnTengwaTwice(tehta)) => {
                f.write_char(base)?;
                self.write_mods(f)?;

                f.write_char(tehta)?;
                f.write_char(tehta)?;
                self.write_rince(f)?;
            }
            None => {
                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince(f)?;
            }
        }

        Ok(())
    }
}

impl From<char> for Glyph {
    fn from(cons: char) -> Self {
        Self::new_base(cons)
    }
}

impl From<Tengwa<'_>> for Glyph {
    fn from(tengwa: Tengwa) -> Self {
        Self::new_base(*tengwa)
    }
}

impl From<Tehta> for Glyph {
    fn from(tehta: Tehta) -> Self {
        Self::new_tehta(tehta)
    }
}
