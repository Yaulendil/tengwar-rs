pub mod beleriand;
pub mod characters;
pub mod etc;
pub mod quenya;
pub mod sindarin;

pub use beleriand::Beleriand;
pub use characters::{int_10, int_12};
pub use quenya::Quenya;
pub use sindarin::Sindarin;
use std::{borrow::Cow, fmt::{self, Write}};


pub trait Rules {
    fn tokens(input: impl AsRef<str>) -> Vec<Token>;

    fn transcribe(input: impl AsRef<str>) -> String {
        Self::tokens(input).iter().map(|t| t.to_string()).collect::<String>()
    }
}


pub trait ToTengwar {
    fn to_tengwar<R: Rules>(&self) -> String;
}


impl<T: AsRef<str>> ToTengwar for T {
    /// Transliterate this text into the Tengwar.
    #[cfg(not(feature = "ligatures-zwj"))]
    fn to_tengwar<R: Rules>(&self) -> String {
        R::transcribe(self)
    }

    /// Transliterate this text into the Tengwar. A post-processor will run over
    ///     it to insert zero-width joiners and create ligatures where possible.
    ///     Joiner characters are inserted between all `Tengwa` tokens. This
    ///     should have no effect in most places, but any pairs of tengwar that
    ///     can be joined will be.
    //  TODO: Create reference list of ligatures instead. This bruteforce method
    //      is a wasteful hack.
    #[cfg(feature = "ligatures-zwj")]
    fn to_tengwar<R: Rules>(&self) -> String {
        const ZWJ: char = '‍';

        let mut iter = R::tokens(self).into_iter().peekable();
        let mut post: String = String::new();

        while let Some(a) = iter.next() {
            write!(post, "{}", a).expect("Error writing String");

            if matches!(a, Token::Tengwa { .. })
                && matches!(iter.peek(),Some(Token::Tengwa { .. }))
            {
                post.push(ZWJ);
            }
        }

        post
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
