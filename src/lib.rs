pub mod characters;
pub mod etc;
pub mod quenya;
pub mod sindarin;

pub use characters::{int_10, int_12};
pub use quenya::Quenya;
pub use sindarin::Sindarin;
use std::{borrow::Cow, fmt::{self, Write}};


pub trait Rules {
    fn transcribe(input: impl AsRef<str>) -> String;
}


pub trait ToTengwar {
    fn to_tengwar<R: Rules>(&self) -> String;
}


impl<T: AsRef<str>> ToTengwar for T {
    fn to_tengwar<R: Rules>(&self) -> String {
        R::transcribe(self)
    }
}


pub enum Token {
    Char(char),
    String(Cow<'static, str>),
    Tengwa(characters::Glyph),
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(chr) => f.write_char(*chr),
            Self::String(s) => f.write_str(&s),
            Self::Tengwa(t) => t.fmt(f),
        }
    }
}
