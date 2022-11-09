use std::ops::Index;
use super::Tengwa;


/// The Témar are the four series of the regular tengwar. Each Téma is composed
///     of eight Tyeller, each modifying the tengwa in a different way, and is
///     named after its base tengwa.
///
/// Only the first six Tyeller are used in Quenya.
#[derive(Clone, Copy, Debug)]
pub struct Tema {
    /// A hanging Telco, and one Lúva.
    ///     Typically represents a voiceless plosive.
    pub single_dn: char,
    /// A hanging Telco, and two Lúvar.
    ///     Typically represents a voiced plosive.
    pub double_dn: char,
    /// A raised Telco, and one Lúva.
    ///     Typically represents a voiceless fricative.
    pub single_up: char,
    /// A raised Telco, and two Lúvar.
    ///     Typically represents either a voiced fricative or a nasalized
    ///     voiceless plosive.
    pub double_up: char,
    /// A short Telco, and two Lúvar.
    ///     Typically represents a nasal long.
    pub double_sh: char,
    /// A short Telco, and one Lúva.
    ///     Typically represents a nasal short.
    pub single_sh: char,
    /// An extended Telco, and one Lúva.
    ///     Not used in canonical sources.
    pub single_ex: char,
    /// An extended Telco, and two Lúvar.
    ///     Not used in canonical sources.
    pub double_ex: char,
}

impl Tema {
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
    pub const fn new(tema: &'t Tema) -> Self {
        Self { tema, tyelle: Tyelle::new() }
    }

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

    pub const fn as_char(&self) -> &char {
        self.tema.get_char(self.tyelle)
    }

    pub const fn wrapped(self) -> Tengwa<'t> {
        Tengwa::Regular(self)
    }

    pub const fn double(mut self) -> Self {
        self.tyelle = self.tyelle.double();
        self
    }

    pub const fn single(mut self) -> Self {
        self.tyelle = self.tyelle.single();
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

    pub const fn short(mut self) -> Self {
        self.tyelle = self.tyelle.short();
        self
    }

    pub const fn extended(mut self) -> Self {
        self.tyelle = self.tyelle.extended();
        self
    }
}

impl<'t> AsRef<char> for TengwaRegular<'t> {
    fn as_ref(&self) -> &char { self.as_char() }
}

impl<'t> From<TengwaRegular<'t>> for char {
    fn from(tengwa: TengwaRegular<'t>) -> Self { *tengwa.as_char() }
}


/// A small type to represent the specific shape of a regular Tengwa.
#[derive(Clone, Copy, Debug)]
pub struct Tyelle {
    pub stem_dn: bool,
    pub stem_up: bool,
    pub doubled: bool,
}

impl Tyelle {
    pub const fn new() -> Self {
        Self {
            stem_dn: true,
            stem_up: false,
            doubled: false,
        }
    }

    pub const fn is_ascending(&self) -> bool { self.stem_up && !self.stem_dn }
    pub const fn is_descending(&self) -> bool { !self.stem_up && self.stem_dn }
    pub const fn is_short(&self) -> bool { !self.stem_up && !self.stem_dn }
    pub const fn is_extended(&self) -> bool { self.stem_up && self.stem_dn }

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

    pub const fn double(mut self) -> Self {
        self.doubled = true;
        self
    }

    pub const fn single(mut self) -> Self {
        self.doubled = false;
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

    pub const fn short(mut self) -> Self {
        self.stem_dn = false;
        self.stem_up = false;
        self
    }

    pub const fn extended(mut self) -> Self {
        self.stem_dn = true;
        self.stem_up = true;
        self
    }

    pub fn make_double(&mut self) {
        *self = self.double();
    }

    pub fn make_single(&mut self) {
        *self = self.single();
    }

    pub fn make_ascending(&mut self) {
        *self = self.ascending();
    }

    pub fn make_descending(&mut self) {
        *self = self.descending();
    }

    pub fn make_short(&mut self) {
        *self = self.short();
    }

    pub fn make_extended(&mut self) {
        *self = self.extended();
    }
}

impl Default for Tyelle {
    fn default() -> Self { Self::new() }
}
