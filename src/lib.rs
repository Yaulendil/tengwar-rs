pub mod characters;
mod quenya;

pub use quenya::Quenya;
use std::fmt;


pub trait Rules {
    fn transcribe(input: String) -> String;

    /*fn transcribe(input: String) -> String {
        let chars: Vec<char> = input.chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let mut line: &[char] = chars.as_slice();
        let mut out: Vec<Token> = Vec::new();
        let mut teng: Option<Glyph> = None;

        'check_line:
        while !line.is_empty() {
            let mut len: usize = 3;

            'check_sub:
            while len > 0 {
                let _sub = &line[0..len];

                if let Some(_current) = &mut teng {
                    //
                } else {
                    //
                }

                len -= 1;
            }

            if let Some(g) = teng.take() { out.push(Token::Tengwa(g)); }
            out.push(Token::Pass(line[0]));
            line = &line[1..];
        }

        out.iter().map(|t| t.to_string()).collect()
    }*/
}


enum Token {
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
