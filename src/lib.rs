pub mod characters;
mod quenya;

pub use quenya::Quenya;
use std::fmt;


pub trait Rules {
    fn transcribe(input: impl AsRef<str>) -> String;
}


pub trait ToTengwar {
    fn to_tengwar<R: Rules>(&self) -> String;
}


impl<T: AsRef<str>> ToTengwar for T {
    fn to_tengwar<R: Rules>(&self) -> String {
        Quenya::transcribe(self)
    }
}


pub enum Token {
    Pass(char),
    Tengwa(characters::Glyph),
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass(s) => s.fmt(f),
            Self::Tengwa(s) => s.fmt(f),
        }
    }
}
