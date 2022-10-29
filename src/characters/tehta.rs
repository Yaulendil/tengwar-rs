use std::fmt::{Formatter, Write};
use super::consts::carrier;


/// A diacritical vowel marker that may be rendered in an alternate "long" form.
#[derive(Clone, Copy, Debug)]
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
    /// Write this Tehta into a Formatter. The provided boolean argument will
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

    /// Returns true if the long variant of this Tehta would be written with the
    ///     extended "Ára" Telco.
    pub const fn uses_ara(&self) -> bool {
        matches!(self, Self::Single(..))
    }
}
