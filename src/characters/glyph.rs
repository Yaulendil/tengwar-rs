use std::fmt::{Display, Formatter, Write};
use super::*;


/// A single base tengwa, and all of its modifications. This includes the tehta
///     marking, flags for additional diacritics, flags for consonant and vowel
///     length, and an indicator of finality.
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

    pub vowel_style: VowelStyle,

    /// Indicates whether the glyph has a sa-rincë attached.
    pub rince: bool,
    /// Indicates whether the glyph may use a more ornate rincë.
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

    pub dot_inner: bool,
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
            vowel_style: VowelStyle::DEFAULT,

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

    pub const fn is_short_carrier(&self) -> bool {
        match self {
            Self { base: None, tehta: None, .. } => true,
            Self { base: None, tehta_alt: false, .. } => true,
            Self { .. } => false,
        }
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

    pub fn replace_base(&mut self, old: char, new: char) -> bool {
        if self.base == Some(old) {
            self.base = Some(new);
            true
        } else {
            false
        }
    }

    pub fn replace_tehta(&mut self, old: Tehta, new: Tehta) -> bool {
        if self.tehta == Some(old) {
            self.tehta = Some(new);
            true
        } else {
            false
        }
    }

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
            } if nuquerna_valid(base) && !(tehta_alt && tehta.uses_ara()) => {
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

    pub const fn can_take_rince(&self) -> bool {
        !self.rince && rince_valid(self.base())
    }

    pub const fn ligates_with(&self, other: &Self) -> bool {
        ligature_valid(self, other)
    }

    pub const fn telco_ligates(&self) -> bool {
        telco_ligates_with(self.base())
    }
}

impl Display for Glyph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base: char = self.base();

        let mut long: bool = self.tehta_alt && self.base.is_some();
        let nuquerna_ignored: bool = !self.nuquerna && nuquerna_valid(base);

        let tehta_post: Option<&Tehta> = match &self.tehta {
            Some(tehta) if long && self.tehta_first => {
                //  This tehta is a long vowel, and represents the preceding
                //      vowel.
                if tehta.uses_ara() {
                    //  This tehta will go on a long carrier. It therefore needs
                    //      to be written before the base.
                    tehta.write(f, true)?;
                    None
                } else if nuquerna_ignored {
                    //  This tengwa will not be be Nuquerna, but it could have
                    //      been. This means that it cannot host a lengthened
                    //      tehta, and so the vowel should be pushed onto a
                    //      preceding long carrier.
                    f.write_char(carrier(true))?;
                    tehta.write(f, false)?;
                    None
                } else {
                    //  This tehta will be displayed on the tengwa, and so must
                    //      still be written after the base character.
                    Some(tehta)
                }
            }
            Some(tehta) => Some(tehta),
            None => None,
        };

        f.write_char(base)?;

        if self.nasal {
            f.write_char(MOD_NASAL)?;
        }

        if self.long_cons {
            f.write_char(MOD_LONG_CONS)?;
        }

        if self.labial {
            f.write_char(MOD_LABIAL)?;
        }

        if self.palatal {
            f.write_char(MOD_PALATAL)?;
        }

        if self.dot_inner {
            f.write_char(DC_INNER_DOT_1)?;
        }

        if self.dot_under {
            f.write_char(DC_UNDER_DOT_1)?;
        }

        if let Some(tehta) = tehta_post {
            if long {
                if tehta.uses_ara() {
                    //  The vowel tehta will be placed on a following Ára
                    //      carrier. If the base should ligate with Ára, write
                    //      the joiner now.
                    if self.ligate_zwj && ligates_with_ara(base) {
                        f.write_char(ZWJ)?;
                    }
                } else if nuquerna_ignored {
                    //  This tengwa has a Nuquerna variant, but it will not be
                    //      used. However, it also has a long vowel attached,
                    //      which, without intervention, will use a more complex
                    //      diacritic. The long vowel should be put on an Ára
                    //      carrier instead to decrease visual chaos.
                    if self.ligate_zwj && ligates_with_ara(base) {
                        f.write_char(ZWJ)?;
                    }

                    f.write_char(carrier(true))?;
                    long = false;
                }
            }

            tehta.write(f, long)?;
        }

        if self.rince {
            f.write_char(rince(base, self.rince_final))?;
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
