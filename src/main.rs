#[macro_use]
extern crate clap;

use std::{
    io::{BufRead, stdin, stdout, Write},
    process::exit,
};
use tengwar::{Beleriand, Gondor, Quenya, Rules};


#[derive(Debug)]
enum Mode {
    Quenya,
    Gondor,
    Beleriand,
    /*English,*/
}


impl Mode {
    const DEFAULT: Mode = Mode::Quenya;

    fn rules<T: AsRef<str>>(&self, ligatures: bool) -> fn(T) -> String {
        if ligatures {
            match self {
                Mode::Quenya => Quenya::transcribe_with_ligatures,
                Mode::Gondor => Gondor::transcribe_with_ligatures,
                Mode::Beleriand => Beleriand::transcribe_with_ligatures,
                /*Mode::English => English::transcribe_with_ligatures,*/
            }
        } else {
            match self {
                Mode::Quenya => Quenya::transcribe,
                Mode::Gondor => Gondor::transcribe,
                Mode::Beleriand => Beleriand::transcribe,
                /*Mode::English => English::transcribe,*/
            }
        }
    }
}


#[derive(Args, Debug)]
#[command(next_help_heading = "Mode Options")]
struct ModeFlags {
    /// Transliterate in the Classical Mode (default).
    ///
    /// Independent Tengwar represent consonant sounds, with vowels being
    ///     represented by a Tehta placed above either the preceding consonant
    ///     or a "carrier" mark.
    ///
    /// This mode is typically used for Quenya.
    #[arg(long, short)]
    #[arg(group = "mode")]
    quenya: bool,

    /// Transliterate in the Mode of Gondor (experimental).
    ///
    /// Independent Tengwar represent consonant sounds, with vowels being
    ///     represented by a Tehta placed above either the following consonant
    ///     or a "carrier" mark.
    ///
    /// This mode was used for Sindarin during the third age, throughout many of
    ///     the western regions of Middle-earth.
    #[arg(long, short)]
    #[arg(group = "mode")]
    gondor: bool,

    /// Transliterate in the Mode of Beleriand (experimental).
    ///
    /// Independent Tengwar are used for both consonants and vowels. Tehtar are
    ///     used only to mark diphthongs and long vowels. This is also referred
    ///     to as the "full" writing mode.
    ///
    /// This mode was used for Sindarin in Beleriand during the first age, as
    ///     well as in Eregion during the second age.
    #[arg(long, short)]
    #[arg(group = "mode")]
    beleriand: bool,

    /*/// Transliterate in an Orthographic English mode.
    #[arg(long, short)]
    #[arg(group = "mode")]
    english: bool,*/
}


/// Transliterate text into the Tengwar of Fëanáro Finwion.
///
/// Since the Tengwar are simply a writing system, and not a full language,
/// there are various "modes" that can be used for transliteration. The default
/// is the Classical Mode, mainly used for Quenya, but others are available for
/// selection by command line options.
#[derive(Debug, Parser)]
#[command(version, max_term_width(100))]
struct Command {
    /// Use zero-width joiners to ligate output.
    ///
    /// In certain typefaces, a zero-width joiner character may be used to form
    ///     ligatures of Tengwar. This option will add joiners into the output
    ///     text between certain characters.
    ///
    /// For typefaces that do not support these ligatures, the presence of the
    ///     joiners should not affect rendering; However, it does increase the
    ///     number of bytes in the output by approximately 15%.
    #[arg(long, short)]
    ligatures: bool,

    /// Text to be transliterated.
    ///
    /// If this is not provided, Standard Input will be used instead.
    text: Vec<String>,

    #[command(flatten)]
    mode_flags: ModeFlags,
}

impl Command {
    const fn mode(&self) -> Mode {
        if self.mode_flags.quenya {
            Mode::Quenya
        } else if self.mode_flags.gondor {
            Mode::Gondor
        } else if self.mode_flags.beleriand {
            Mode::Beleriand
        /*} else if self.mode_flags.english {
            Mode::English*/
        } else {
            Mode::DEFAULT
        }
    }
}


fn main() {
    let command: Command = clap::Parser::parse();
    let convert: fn(String) -> String = command.mode().rules(command.ligatures);

    if command.text.is_empty() {
        for line in stdin().lock().lines() {
            if let Ok(text) = line {
                println!("{}", convert(text));
            }
        }
    } else {
        let text: String = command.text.join(" ");

        print!("{}", convert(text));
        exit(stdout().write(b"\n").is_err() as i32);
    }
}


#[test]
fn verify_cli() {
    <Command as clap::CommandFactory>::command().debug_assert();
}
