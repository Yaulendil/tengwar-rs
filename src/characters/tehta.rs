use std::fmt::{Formatter, Write};
use super::consts::carrier;


/// A diacritical vowel marker that may be rendered in an alternate "long" form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tehta {
    /// There is only one form that this diacritic can take. The long form must
    ///     be rendered with an extended carrier mark.
    Single(char),
    /// There is one form that this diacritic can take, but it may be repeated
    ///     for the long form.
    Double(char),
    /// This diacritic has an alternate character to represent its long form.
    Altern(char, char),
}

impl Tehta {
    pub const fn has_alt(&self) -> bool {
        matches!(self, Self::Altern(..))
    }

    pub const fn is_double(&self) -> bool {
        matches!(self, Self::Double(..))
    }

    pub const fn alt(&self) -> Option<char> {
        match self {
            Self::Single(_) => None,
            Self::Double(_) => None,
            Self::Altern(_, mark) => Some(*mark),
        }
    }

    /// Return the basic mark, used for short vowels.
    pub const fn base(&self) -> char {
        match self {
            Self::Single(mark) => *mark,
            Self::Double(mark) => *mark,
            Self::Altern(mark, _) => *mark,
        }
    }

    /// Return the long mark, used for long vowels. It may or may not be the
    ///     same as the base mark.
    pub const fn long(&self) -> char {
        match self {
            Self::Single(mark) => *mark,
            Self::Double(mark) => *mark,
            Self::Altern(_, mark) => *mark,
        }
    }

    /*pub const fn mark(&self, long: bool) -> char {
        match self {
            Self::Single(mark) => *mark,
            Self::Double(mark) => *mark,
            Self::Altern(mark, alt) => if long { *alt } else { *mark }
        }
    }*/

    /// Returns `true` if the long variant of this tehta would be written with
    ///     the extended "Ára" carrier.
    pub const fn uses_ara(&self) -> bool {
        matches!(self, Self::Single(..))
    }

    /// Write this tehta into a Formatter. The provided boolean argument will
    ///     determine whether the basic short form or the variable long form is
    ///     written.
    pub fn write(&self, f: &mut Formatter<'_>, long: bool) -> std::fmt::Result {
        match self {
            &Self::Single(mark) => {
                if long {
                    f.write_char(carrier(true))?;
                }

                f.write_char(mark)
            }
            &Self::Double(mark) => {
                if long {
                    f.write_char(mark)?;
                }

                f.write_char(mark)
            }
            &Self::Altern(mark, alt) => {
                if long {
                    f.write_char(alt)
                } else {
                    f.write_char(mark)
                }
            }
        }
    }
}
