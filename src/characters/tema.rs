//! There are four "series" of regular tengwar, called the Témar. Each Téma is
//!     divided into eight "grades" or "orders", called the Tyeller. Together,
//!     the Témar and the Tyeller form a neat table of 32 regular tengwar. This
//!     module defines the types needed to represent this table in detail.
//!
//! NOTE: Canonical names are used throughout this project, but especially here,
//!     in order to prevent ambiguity; The term "series" may be taken to refer
//!     to a sequence of tengwar or tokens, but a "Téma" can only be a Téma.

use std::ops::{/*Deref, DerefMut,*/ Index};
use super::Tengwa;


/// The Témar are the four series of the regular tengwar. Each Téma is composed
///     of eight Tyeller, each modifying the tengwa in a different way, and is
///     named after its base tengwa.
///
/// Only the first six Tyeller are used in Quenya.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Tema {
    /// Whether the bows of these characters face left.
    pub left: bool,
    /// Whether the bows of these characters are open.
    pub open: bool,

    /// A descending stem with one bow.
    pub single_dn: char,
    /// A descending stem with two bows.
    pub double_dn: char,
    /// An ascending stem with one bow.
    pub single_up: char,
    /// An ascending stem with two bows.
    pub double_up: char,
    /// A short stem with two bows.
    pub double_sh: char,
    /// A short stem with one bow.
    pub single_sh: char,
    /// An extended stem with one bow.
    pub single_ex: char,
    /// An extended stem with two bows.
    pub double_ex: char,
}

impl Tema {
    /// Determine whether a [`char`] is part of this Téma.
    pub const fn contains(&self, c: char) -> bool {
         c == self.single_dn || c == self.double_dn
             || c == self.single_up || c == self.double_up
             || c == self.double_sh || c == self.single_sh
             || c == self.single_ex || c == self.double_ex
    }

    /// Return the [`TengwaRegular`] in this Téma with the default [`Tyelle`].
    pub const fn new_tengwa(&self) -> TengwaRegular {
        TengwaRegular::new(self)
    }

    /// Return the [`TengwaRegular`] in this Téma at a specified [`Tyelle`].
    pub const fn get_tengwa(&self, tyelle: Tyelle) -> TengwaRegular {
        TengwaRegular { tema: self, tyelle }
    }

    /// Return the [`char`] in this Téma at a specified [`Tyelle`].
    pub const fn get_char(&self, tyelle: Tyelle) -> &char {
        const Y: bool = true;
        const N: bool = false;
        match tyelle {
            Tyelle { stem_dn: Y, stem_up: N, doubled: N } => &self.single_dn,
            Tyelle { stem_dn: Y, stem_up: N, doubled: Y } => &self.double_dn,
            Tyelle { stem_dn: N, stem_up: Y, doubled: N } => &self.single_up,
            Tyelle { stem_dn: N, stem_up: Y, doubled: Y } => &self.double_up,
            Tyelle { stem_dn: N, stem_up: N, doubled: Y } => &self.double_sh,
            Tyelle { stem_dn: N, stem_up: N, doubled: N } => &self.single_sh,
            Tyelle { stem_dn: Y, stem_up: Y, doubled: N } => &self.single_ex,
            Tyelle { stem_dn: Y, stem_up: Y, doubled: Y } => &self.double_ex,
        }
    }

    /// Find the [`Tyelle`] of a given [`char`] in this Téma.
    pub const fn find_tyelle(&self, c: char) -> Option<Tyelle> {
        let tyelle: Tyelle = if c == self.single_dn {
            Tyelle::new().single().descending()
        } else if c == self.double_dn {
            Tyelle::new().double().descending()
        } else if c == self.single_up {
            Tyelle::new().single().ascending()
        } else if c == self.double_up {
            Tyelle::new().double().ascending()
        } else if c == self.double_sh {
            Tyelle::new().double().short()
        } else if c == self.single_sh {
            Tyelle::new().single().short()
        } else if c == self.single_ex {
            Tyelle::new().single().extended()
        } else if c == self.double_ex {
            Tyelle::new().double().extended()
        } else {
            return None;
        };

        // debug_assert_eq!(self.get_char(tyelle), &c);
        Some(tyelle)
    }
}

impl Index<Tyelle> for Tema {
    type Output = char;
    fn index(&self, tyelle: Tyelle) -> &Self::Output { self.get_char(tyelle) }
}


/// A small type pairing a [`Tema`] with a specific [`Tyelle`].
#[derive(Clone, Copy, Debug)]
pub struct TengwaRegular<'t> {
    pub tema: &'t Tema,
    pub tyelle: Tyelle,
}

impl<'t> TengwaRegular<'t> {
    /// Define a new tengwa that is part of a given [`Tema`].
    pub const fn new(tema: &'t Tema) -> Self {
        Self { tema, tyelle: Tyelle::new() }
    }

    /// Try to find the tengwa corresponding to a given [`char`] within the four
    ///     primary Témar.
    pub const fn find(char: char) -> Option<Self> {
        if let Some(tyelle) = Tema::TINCO.find_tyelle(char) {
            Some(Tema::TINCO.get_tengwa(tyelle))
        } else if let Some(tyelle) = Tema::PARMA.find_tyelle(char) {
            Some(Tema::PARMA.get_tengwa(tyelle))
        } else if let Some(tyelle) = Tema::CALMA.find_tyelle(char) {
            Some(Tema::CALMA.get_tengwa(tyelle))
        } else if let Some(tyelle) = Tema::QESSE.find_tyelle(char) {
            Some(Tema::QESSE.get_tengwa(tyelle))
        } else {
            None
        }
    }

    /// Return the true [`char`] represented by this tengwa.
    pub const fn as_char(&self) -> &char {
        self.tema.get_char(self.tyelle)
    }

    /// Wrap this regular tengwa in a [`Tengwa`] enum, allowing interoperation
    ///     with irregulars.
    pub const fn wrapped(self) -> Tengwa<'t> {
        Tengwa::Regular(self)
    }

    pub const fn single(mut self) -> Self {
        self.tyelle = self.tyelle.single();
        self
    }

    pub const fn double(mut self) -> Self {
        self.tyelle = self.tyelle.double();
        self
    }

    pub const fn ascending(mut self) -> Self {
        self.tyelle = self.tyelle.ascending();
        self
    }

    pub const fn descending(mut self) -> Self {
        self.tyelle = self.tyelle.descending();
        self
    }

    pub const fn extended(mut self) -> Self {
        self.tyelle = self.tyelle.extended();
        self
    }

    pub const fn short(mut self) -> Self {
        self.tyelle = self.tyelle.short();
        self
    }
}

impl<'t> AsRef<char> for TengwaRegular<'t> {
    fn as_ref(&self) -> &char { self.as_char() }
}

/*impl<'t> Deref for TengwaRegular<'t> {
    type Target = Tyelle;
    fn deref(&self) -> &Self::Target { &self.tyelle }
}

impl<'t> DerefMut for TengwaRegular<'t> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.tyelle }
}*/

impl<'t> From<TengwaRegular<'t>> for char {
    fn from(tengwa: TengwaRegular<'t>) -> Self { *tengwa.as_char() }
}


/// A small type to represent the specific shape of a regular Tengwa.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Tyelle {
    pub stem_dn: bool,
    pub stem_up: bool,
    pub doubled: bool,
}

impl Tyelle {
    /// Define a new [`Tyelle`], starting at the first order.
    pub const fn new() -> Self {
        Self {
            stem_dn: true,
            stem_up: false,
            doubled: false,
        }
    }

    pub const fn is_double(&self) -> bool { self.doubled }
    pub const fn is_ascending(&self) -> bool { self.stem_up && !self.stem_dn }
    pub const fn is_descending(&self) -> bool { !self.stem_up && self.stem_dn }
    pub const fn is_extended(&self) -> bool { self.stem_up && self.stem_dn }
    pub const fn is_short(&self) -> bool { !self.stem_up && !self.stem_dn }

    /// Return the [`TengwaRegular`] found at this Tyellë on a given [`Tema`].
    pub const fn on_tema<'t>(&self, tema: &'t Tema) -> TengwaRegular<'t> {
        TengwaRegular { tema, tyelle: *self }
    }

    //  TODO: Would this be of any actual use? Being a bitfield would lead to a
    //      size of only one byte, while also allowing a Téma to be represented
    //      as an array with its Tyeller as simple integer indices. However, it
    //      is hard to imagine that being a significant speedup, even for the
    //      TeX macro use case.
    /*pub const fn bits(self) -> u8 {
        0u8 | self.stem_dn as u8 * 0b001
            | self.stem_up as u8 * 0b010
            | self.doubled as u8 * 0b100
    }*/

    pub const fn single(mut self) -> Self {
        self.doubled = false;
        self
    }

    pub const fn double(mut self) -> Self {
        self.doubled = true;
        self
    }

    pub const fn ascending(mut self) -> Self {
        self.stem_dn = false;
        self.stem_up = true;
        self
    }

    pub const fn descending(mut self) -> Self {
        self.stem_dn = true;
        self.stem_up = false;
        self
    }

    pub const fn extended(mut self) -> Self {
        self.stem_dn = true;
        self.stem_up = true;
        self
    }

    pub const fn short(mut self) -> Self {
        self.stem_dn = false;
        self.stem_up = false;
        self
    }

    pub fn make_single(&mut self) {
        *self = self.single();
    }

    pub fn make_double(&mut self) {
        *self = self.double();
    }

    pub fn make_ascending(&mut self) {
        *self = self.ascending();
    }

    pub fn make_descending(&mut self) {
        *self = self.descending();
    }

    pub fn make_extended(&mut self) {
        *self = self.extended();
    }

    pub fn make_short(&mut self) {
        *self = self.short();
    }
}

impl Default for Tyelle {
    fn default() -> Self { Self::new() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tyeller() {
        let as_1 = Tyelle::new().ascending().single();
        assert!(!as_1.is_double());
        assert!(as_1.is_ascending());
        assert!(!as_1.is_descending());
        assert!(!as_1.is_extended());
        assert!(!as_1.is_short());

        let as_2 = Tyelle::new().ascending().double();
        assert!(as_2.is_double());
        assert!(as_2.is_ascending());
        assert!(!as_2.is_descending());
        assert!(!as_2.is_extended());
        assert!(!as_2.is_short());

        let de_1 = Tyelle::new().descending().single();
        assert!(!de_1.is_double());
        assert!(!de_1.is_ascending());
        assert!(de_1.is_descending());
        assert!(!de_1.is_extended());
        assert!(!de_1.is_short());

        let de_2 = Tyelle::new().descending().double();
        assert!(de_2.is_double());
        assert!(!de_2.is_ascending());
        assert!(de_2.is_descending());
        assert!(!de_2.is_extended());
        assert!(!de_2.is_short());

        let ex_1 = Tyelle::new().extended().single();
        assert!(!ex_1.is_double());
        assert!(!ex_1.is_ascending());
        assert!(!ex_1.is_descending());
        assert!(ex_1.is_extended());
        assert!(!ex_1.is_short());

        let ex_2 = Tyelle::new().extended().double();
        assert!(ex_2.is_double());
        assert!(!ex_2.is_ascending());
        assert!(!ex_2.is_descending());
        assert!(ex_2.is_extended());
        assert!(!ex_2.is_short());

        let sh_1 = Tyelle::new().short().single();
        assert!(!sh_1.is_double());
        assert!(!sh_1.is_ascending());
        assert!(!sh_1.is_descending());
        assert!(!sh_1.is_extended());
        assert!(sh_1.is_short());

        let sh_2 = Tyelle::new().short().double();
        assert!(sh_2.is_double());
        assert!(!sh_2.is_ascending());
        assert!(!sh_2.is_descending());
        assert!(!sh_2.is_extended());
        assert!(sh_2.is_short());
    }
}
