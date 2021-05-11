pub mod beleriand;
pub mod characters;
pub mod etc;
pub mod quenya;
pub mod sindarin;

pub use beleriand::Beleriand;
pub use characters::{Glyph, int_10, int_12, ligature_valid};
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
    ///     This affects the text data itself, but should not have any visible
    ///     effect with a font that does not support the ligatures.
    #[cfg(feature = "ligatures-zwj")]
    fn to_tengwar<R: Rules>(&self) -> String {
        use characters::ZWJ;

        let mut iter = R::tokens(self).into_iter().peekable();
        let mut post: String = String::new();

        while let Some(token) = iter.next() {
            write!(post, "{}", token).expect("Error writing String");

            if let Token::Tengwa(prev) = token {
                if let Some(Token::Tengwa(next)) = iter.peek() {
                    if ligature_valid(&prev, &next) {
                        post.push(ZWJ);
                    }
                }
            }
        }

        post
    }
}


pub enum Token {
    Char(char),
    String(Cow<'static, str>),
    Tengwa(Glyph),
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(chr) => f.write_char(*chr),
            Self::String(s) => f.write_str(&s),
            Self::Tengwa(t) => <Glyph as fmt::Display>::fmt(t, f),
        }
    }
}
