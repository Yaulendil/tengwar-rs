use crate::{characters::*, Rules, Token};


pub struct Quenya;


impl Rules for Quenya {
    fn transcribe(input: String) -> String {
        let cvec: Vec<char> = input.chars()
            .map(|c| c.to_ascii_lowercase())
            .collect();
        let mut line: &[char] = cvec.as_slice();
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
    }
}
