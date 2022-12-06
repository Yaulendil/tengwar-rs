use std::{fmt::{Display, Formatter, Write}, marker::PhantomData};
use crate::policy::{Policy, Standard};
use super::*;


/// An optional base [`Tengwa`] paired with an optional diacritical tehta.
#[derive(Clone, Copy, Debug)]
pub struct TengwaTehta<'t> {
    /// An optional base [`Tengwa`]. If this is not present, a carrier mark
    ///     should be used.
    pub tengwa: Option<Tengwa<'t>>,
    /// An optional [`char`] to represent a diacritical tehta, which should be
    ///     displayed directly above the base tengwa.
    pub tehta: Option<char>,
}


/// A representation of the visual composition of a [`Glyph`].
#[derive(Clone, Copy, Debug)]
pub enum Parts<'t> {
    /// This glyph can be displayed with only a single base character.
    One(TengwaTehta<'t>),
    /// This glyph requires two base characters to be displayed.
    Two(TengwaTehta<'t>, TengwaTehta<'t>),
}

impl<'t> Parts<'t> {
    /// Returns `true` if there are two separate base characters.
    pub const fn has_two(&self) -> bool {
        match self {
            Self::One(..) => false,
            Self::Two(..) => true,
        }
    }

    /// Returns the left-hand [`TengwaTehta`], if there are two parts; If there
    ///     is only one part, returns the one.
    pub const fn lhs(&self) -> &TengwaTehta<'t> {
        match self {
            Self::One(tt) => tt,
            Self::Two(tt, _) => tt,
        }
    }

    /// Returns the right-hand [`TengwaTehta`], if there are two parts; If there
    ///     is only one part, returns the one.
    pub const fn rhs(&self) -> &TengwaTehta<'t> {
        match self {
            Self::One(tt) => tt,
            Self::Two(_, tt) => tt,
        }
    }
}


/// The specific [`char`] to be used for a tehta, with a specific position to
///     display it.
#[derive(Clone, Copy, Debug)]
pub enum TehtaChar {
    /// The tehta will be placed on a [long carrier](CARRIER_LONG) following the
    ///     base character.
    OnAraAfter(char),
    /// The tehta will be placed on a [long carrier](CARRIER_LONG) preceding the
    ///     base character.
    OnAraBefore(char),
    /// The tehta will be placed on the base character itself.
    OnTengwaOnce(char),
    /// The tehta will be placed *twice* on the base character itself.
    OnTengwaTwice(char),
}

impl TehtaChar {
    /// Returns `true` if the tehta will not be placed above the base character.
    pub const fn is_separate(&self) -> bool {
        match self {
            Self::OnAraAfter(_) => true,
            Self::OnAraBefore(_) => true,
            Self::OnTengwaOnce(_) => false,
            Self::OnTengwaTwice(_) => false,
        }
    }
}


fn write_tehta(f: &mut Formatter<'_>, tehta: char, double: bool) -> std::fmt::Result {
    if double {
        f.write_char(tehta)?;
        f.write_char(tehta)?;
    } else {
        f.write_char(tehta)?;
    }

    Ok(())
}


/// A single base tengwa, and all of its modifications. This includes the tehta
///     marking, flags for additional diacritics, flags for consonant and vowel
///     length, and information on vowel and ligature behavior.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Glyph<P: Policy = Standard> {
    /// A base character.
    pub base: Option<char>,
    /// The primary diacritical marking over the base character.
    pub tehta: Option<Tehta>,
    /// Indicates whether the [`Tehta`] should use its alternate "long" form.
    pub tehta_alt: bool,
    /// Indicates whether a tehta with an extended carrier should be printed
    ///     before the glyph.
    pub tehta_first: bool,
    pub tehta_hidden: bool,

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

    /// Indicates whether this glyph should try to use [`ZWJ`] ligation.
    pub ligate_zwj: u8,

    /// Phantom field to carry the [`Policy`] parameter.
    pub _p: PhantomData<P>,
}

/// Public: Construction and modification.
impl<P: Policy> Glyph<P> {
    /// Define a new empty glyph.
    pub const fn new() -> Self {
        Self {
            base: None,
            tehta: None,
            tehta_alt: false,
            tehta_first: false,
            tehta_hidden: false,
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
            ligate_zwj: 0,
            _p: PhantomData,
        }
    }

    //  TODO: Reduce number of constructors.
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

    /// Switch the glyph to use a different [`Policy`] implementor.
    pub const fn change_policy<Q: Policy>(&self) -> Glyph<Q> {
        Glyph {
            base: self.base,
            tehta: self.tehta,
            tehta_alt: self.tehta_alt,
            tehta_first: self.tehta_first,
            tehta_hidden: self.tehta_hidden,
            vowels: self.vowels,
            rince: self.rince,
            rince_final: self.rince_final,
            nasal: self.nasal,
            labial: self.labial,
            palatal: self.palatal,
            nuquerna: self.nuquerna,
            long_cons: self.long_cons,
            dot_inner: self.dot_inner,
            dot_under: self.dot_under,
            ligate_short: self.ligate_short,
            ligate_zwj: self.ligate_zwj,
            _p: PhantomData,
        }
    }

    /// Change the base [`char`].
    pub const fn with_tengwa(mut self, tengwa: char) -> Self {
        self.base = Some(tengwa);
        self
    }

    /// Change the [`Tehta`] to be used.
    pub const fn with_tehta(mut self, tehta: Tehta) -> Self {
        self.tehta = Some(tehta);
        self
    }

    /// Mark this glyph as using the alternate form of its [`Tehta`].
    pub const fn with_tehta_alt(mut self, enabled: bool) -> Self {
        self.tehta_alt = enabled;
        self
    }

    /// Mark this glyph as being underlined.
    pub const fn with_underline(mut self, enabled: bool) -> Self {
        self.long_cons = enabled;
        self
    }

    /// Mark this glyph as being labialized. It will be rendered with a wavy
    ///     overbar.
    pub const fn with_labial(mut self, enabled: bool) -> Self {
        self.labial = enabled;
        self
    }

    /// Mark this glyph as being nasalized. It will be rendered overlined.
    pub const fn with_nasal(mut self, enabled: bool) -> Self {
        self.nasal = enabled;
        self
    }

    /// Mark this glyph as being palatalized. It will be rendered with a pair of
    ///     dots below it.
    pub const fn with_palatal(mut self, enabled: bool) -> Self {
        self.palatal = enabled;
        self
    }

    /// Mark this glyph as being followed by a sibilant. It may be rendered with
    ///     a flourish.
    pub const fn with_rince(mut self, enabled: bool) -> Self {
        self.rince = enabled;
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

/// Public: Information and logic.
impl<P: Policy> Glyph<P> {
    /// Determine the base character to be used for this glyph. If one is not
    ///     set, an appropriate "carrier" mark will be returned instead.
    pub fn base_nuq(&self) -> char {
        match self {
            &Glyph {
                base: Some(base),
                tehta: Some(tehta),
                tehta_alt,
                nuquerna: true,
                vowels: VowelStyle::Doubled | VowelStyle::Unique,
                ..
            } if P::nuquerna_valid(base) && !(tehta_alt && tehta.needs_ara()) => {
                //  In this case, ALL of the following are true:
                //    - The glyph has both a tengwa and a tehta.
                //    - The base tengwa has a Nuquerna variant.
                //    - The glyph is set to use the Nuquerna variant.
                //    - The tehta will be displayed on the tengwa.
                //  The Nuquerna variant of the base will therefore be returned.
                P::nuquerna(base)
            }

            _ => self.base(),
        }
    }

    /// Determine the base character to be used for this glyph. If one is not
    ///     set, an appropriate "carrier" mark will be returned instead.
    ///
    /// This method does not apply a Nuquerna variant.
    pub const fn base(&self) -> char {
        match self {
            &Glyph { base: Some(base), .. } => base,
            &Glyph { base: None, tehta_alt, ligate_short, .. } => {
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

    /// Return a [`Tengwa`] representing the base [`char`], if there is one.
    pub const fn tengwa(&self) -> Option<Tengwa<'static>> {
        match self.base {
            Some(char) => Some(Tengwa::either_from(char)),
            None => None,
        }
    }

    /// Return [`Parts`] representing the final visual composition of the glyph.
    pub fn parts(&self) -> Parts<'static> {
        let tehta: Option<TehtaChar> = self.tehta_char();

        match tehta {
            Some(TehtaChar::OnAraAfter(c)) => {
                let lhs = TengwaTehta { tengwa: self.tengwa(), tehta: None };
                let rhs = TengwaTehta { tengwa: None, tehta: Some(c) };
                Parts::Two(lhs, rhs)
            }
            Some(TehtaChar::OnAraBefore(c)) => {
                let lhs = TengwaTehta { tengwa: None, tehta: Some(c) };
                let rhs = TengwaTehta { tengwa: self.tengwa(), tehta: None };
                Parts::Two(lhs, rhs)
            }
            Some(TehtaChar::OnTengwaOnce(c)) => Parts::One(TengwaTehta {
                tengwa: self.tengwa(),
                tehta: Some(c),
            }),
            Some(TehtaChar::OnTengwaTwice(c)) => Parts::One(TengwaTehta {
                tengwa: self.tengwa(),
                tehta: Some(c),
            }),
            None => Parts::One(TengwaTehta {
                tengwa: self.tengwa(),
                tehta: None,
            }),
        }
    }

    /// Determine whether a rincë may be added to this glyph. Returns `false` if
    ///     a rincë is already set.
    pub fn can_take_rince(&self) -> bool {
        !self.rince && P::rince_valid(self.base())
    }

    /// Determine whether the base character has a nuquerna variant, but is set
    ///     to not use it.
    pub fn ignoring_nuquerna(&self) -> bool {
        match self.base {
            Some(base) if !self.nuquerna => P::nuquerna_valid(base),
            _ => false,
        }
    }

    /// Determine whether is glyph has no basic forms at all.
    pub const fn is_empty(&self) -> bool {
        self.base.is_none() && self.tehta.is_none()
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
    pub fn ligates_with<Q: Policy>(&self, other: &Glyph<Q>) -> bool {
        P::ligature_valid(self, other, self.ligate_zwj)
    }

    /// Determine whether the base [`char`] of this glyph is permitted to ligate
    ///     with [Ára](TENGWA_ARA) using a zero-width joiner.
    pub fn ligates_with_ara(&self) -> bool {
        P::ligates_with_ara(self.base())
    }

    /// Determine whether [Telco](TENGWA_TELCO) is permitted to ligate with the
    ///     base [`char`] of this glyph using a zero-width joiner.
    pub fn telco_ligates(&self) -> bool {
        P::telco_ligates_with(self.base())
    }

    /// Choose the correct form of Sa-Rincë.
    pub fn choose_rince(&self) -> Rince {
        P::rince(self.base(), self.rince_final)
    }

    pub fn carries_tehta(&self) -> bool {
        match self.tehta_char() {
            Some(TehtaChar::OnAraAfter(_)) => false,
            Some(TehtaChar::OnAraBefore(_)) => false,
            Some(TehtaChar::OnTengwaOnce(_)) => true,
            Some(TehtaChar::OnTengwaTwice(_)) => true,
            None => false,
        }
    }

    /// Resolve the position and identity of the tehta.
    pub fn tehta_char(&self) -> Option<TehtaChar> {
        let Some(tehta) = self.tehta else {
            //  If there is no tehta, there is nothing to use for it.
            return None;
        };
        let Some(tengwa) = self.base else {
            //  If there is no tengwa, the base will be a carrier already.
            return Some(TehtaChar::OnTengwaOnce(tehta.base));
        };

        let char: char;
        let is_double: bool;
        let needs_ara: bool;

        match (self.vowels, tehta) {
            (VowelStyle::Doubled, Tehta { base, can_double: true, .. }) => {
                char = base;
                is_double = self.tehta_alt;
                needs_ara = false;
            }
            (VowelStyle::Unique, Tehta { base, alternate: Some(alt), .. }) => {
                char = if self.tehta_alt { alt } else { base };
                is_double = false;
                needs_ara = false;
            }
            (_, Tehta { base, .. }) => {
                char = base;
                is_double = false;
                needs_ara = self.tehta_alt;
            }
        }

        //  If the base tengwa has a Nuquerna variant, but it is not going to be
        //      used, the standard form cannot hold a double or alternate tehta.
        let nuq_ignored: bool = !self.nuquerna && P::nuquerna_valid(tengwa);
        let cannot_hold: bool = self.tehta_alt && nuq_ignored;

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

    /// Return `true` if the glyph carries the [A-tehta](TEHTA_A).
    pub fn tehta_is_a(&self) -> bool {
        self.tehta == Some(TEHTA_A)
    }
}

/// Private: Helper methods.
impl<P: Policy> Glyph<P> {
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
            match self.choose_rince() {
                Rince::Basic => f.write_char(SA_RINCE),
                Rince::Final => f.write_char(SA_RINCE_FINAL),
            }
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

    fn write_rince_tehta(
        &self,
        f: &mut Formatter<'_>,
        tehta: char,
        double: bool,
    ) -> std::fmt::Result {
        //  NOTE: If there will be a non-final rincë in this position, write it
        //      BEFORE the tehta. Discovered an issue with the basic rincë, when
        //      placed after the tehta, not combining properly after the unique
        //      long forms.
        if self.rince {
            match self.choose_rince() {
                Rince::Basic => {
                    f.write_char(SA_RINCE)?;
                    if !self.tehta_hidden { write_tehta(f, tehta, double)?; }
                }
                Rince::Final => {
                    if !self.tehta_hidden { write_tehta(f, tehta, double)?; }
                    f.write_char(SA_RINCE_FINAL)?;
                }
            }
        } else {
            if !self.tehta_hidden { write_tehta(f, tehta, double)?; }
        }

        Ok(())
    }
}

impl<P: Policy> Display for Glyph<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base: char = self.base_nuq();

        match self.tehta_char() {
            Some(TehtaChar::OnAraAfter(tehta)) => {
                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince_nonfinal(f)?;

                if 0 < self.ligate_zwj && P::ligates_with_ara(base) {
                    f.write_char(ZWJ)?;
                }

                f.write_char(CARRIER_LONG)?;
                if !self.tehta_hidden { f.write_char(tehta)?; }
            }
            Some(TehtaChar::OnAraBefore(tehta)) => {
                f.write_char(CARRIER_LONG)?;
                if !self.tehta_hidden { f.write_char(tehta)?; }

                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince(f)?;
            }
            Some(TehtaChar::OnTengwaOnce(tehta)) => {
                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince_tehta(f, tehta, false)?;
            }
            Some(TehtaChar::OnTengwaTwice(tehta)) => {
                f.write_char(base)?;
                self.write_mods(f)?;
                self.write_rince_tehta(f, tehta, true)?;
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

impl<P: Policy> From<char> for Glyph<P> {
    fn from(cons: char) -> Self {
        Self::new_base(cons)
    }
}

impl<P: Policy> From<Tengwa<'_>> for Glyph<P> {
    fn from(tengwa: Tengwa) -> Self {
        Self::new_base(*tengwa)
    }
}

impl<P: Policy> From<Tehta> for Glyph<P> {
    fn from(tehta: Tehta) -> Self {
        Self::new_tehta(tehta)
    }
}
