use std::ops::Index;


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
    pub const fn tengwa(&self) -> TengwaRegular {
        TengwaRegular::new(*self)
    }

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
}

impl Index<Tyelle> for Tema {
    type Output = char;
    fn index(&self, tyelle: Tyelle) -> &Self::Output { self.get_char(tyelle) }
}


/// A small type pairing a [`Tema`] with a specific [`Tyelle`].
#[derive(Clone, Copy, Debug)]
pub struct TengwaRegular {
    pub tema: Tema,
    pub tyelle: Tyelle,
}

impl TengwaRegular {
    pub const fn new(tema: Tema) -> Self {
        Self { tema, tyelle: Tyelle::new() }
    }

    pub const fn to_char(&self) -> &char {
        self.tema.get_char(self.tyelle)
    }

    pub const fn double(mut self) -> Self {
        self.tyelle.doubled = true;
        self
    }

    pub const fn single(mut self) -> Self {
        self.tyelle.doubled = false;
        self
    }

    pub const fn ascending(mut self) -> Self {
        self.tyelle.stem_dn = false;
        self.tyelle.stem_up = true;
        self
    }

    pub const fn descending(mut self) -> Self {
        self.tyelle.stem_dn = true;
        self.tyelle.stem_up = false;
        self
    }

    pub const fn short(mut self) -> Self {
        self.tyelle.stem_dn = false;
        self.tyelle.stem_up = false;
        self
    }

    pub const fn extended(mut self) -> Self {
        self.tyelle.stem_dn = true;
        self.tyelle.stem_up = true;
        self
    }
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
}

impl Default for Tyelle {
    fn default() -> Self { Self::new() }
}
