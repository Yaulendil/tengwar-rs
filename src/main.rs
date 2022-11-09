#[macro_use]
extern crate clap;

mod bin_mode;
mod bin_test;

use std::{io::{BufRead, stdin, stdout, Write}, process::exit};
use bin_mode::*;


#[derive(Args, Debug)]
struct ModeFlags {
    /// Transliterate in the Classical Mode (default).
    ///
    /// Independent Tengwar represent consonant sounds, with vowels being
    ///     represented by a Tehta placed above either the preceding consonant
    ///     or a "carrier" mark.
    ///
    /// This mode is typically used for Quenya.
    #[arg(long, short = 'Q')]
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
    #[arg(long, short = 'G')]
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
    #[arg(long, short = 'B')]
    #[arg(group = "mode")]
    beleriand: bool,

    /*/// Transliterate in an Orthographic English mode.
    #[arg(long, short)]
    #[arg(group = "mode")]
    english: bool,*/

    /// Set a mode by name.
    #[arg(hide = true)] // NOTE: Unsure whether this option will be kept.
    #[arg(long = "mode", short = 'M', value_name = "MODE")]
    #[arg(group = "mode", value_enum, ignore_case = true)]
    by_name: Option<Mode>,
    //  TODO: Use as fallback, replacing Mode::DEFAULT, rather than first check?
    // #[arg(default_value_t = Mode::Quenya)]
    // by_name: Mode,
}


#[derive(Clone, Copy, Debug, ValueEnum)]
enum LongVowels {
    /// Always use the extended carrier mark.
    Separate,
    /// Where possible, use doubled diacritics.
    Doubled,
    /// Where possible, use unique diacritics.
    Unique,
}


#[derive(Args, Debug)]
struct StyleFlags {
    /*/// Use an alternate "yanta" diacritic for A-vowels.
    ///
    /// The alternate form is simpler and much quicker to write by hand than the
    ///     default tri-dot, and may be preferred when typesetting text intended
    ///     to be handwritten.
    #[arg(long, short = 'a')]
    alt_a: bool,*/

    /*/// Use a more ornate "sa-rincë" for final sibilants.
    #[arg(long, short = 'r')]
    alt_rince: bool,*/

    /*/// Set behavior for long vowels.
    #[arg(long, short = 'l', value_name = "STYLE")]
    #[arg(default_value_t = LongVowels::Doubled, value_enum)]
    long: LongVowels,*/

    /// Do not use inverted "nuquerna" variants.
    ///
    /// Some tengwar typically occupy the center space above them, where a vowel
    ///     diacritic would be placed. When one of these tengwar needs to have a
    ///     vowel, it is often inverted to make room; This option prevents that.
    #[arg(long, short = 'n')]
    no_nuquernar: bool,
}


/// Transliterate text into J.R.R. Tolkien's Tengwar.
///
/// Since the Tengwar are simply a writing system, and not a full language,
/// there are various "modes" that can be used for transliteration. The default
/// is the Classical Mode, mainly used for Quenya, but others are available for
/// selection by command line options.
#[derive(Debug, Parser)]
#[command(version, max_term_width(100))]
struct Command {
    /// Text to be transliterated.
    ///
    /// If this is not provided, Standard Input will be used instead.
    text: Vec<String>,

    /// Use all available forms of ligature formation.
    #[arg(long)]
    ligate_all: bool,

    /// Use the ligated short carrier when applicable.
    #[arg(long, short = 's')]
    ligate_short: bool,

    /// Use zero-width joiners to ligate output.
    ///
    /// In certain typefaces, a zero-width joiner character may be used to form
    ///     ligatures of Tengwar. This option will add joiners into the output
    ///     text between certain characters.
    ///
    /// For typefaces that do not support these ligatures, the presence of the
    ///     joiners should not affect rendering; However, it does increase the
    ///     number of bytes in the output by approximately 15%.
    #[arg(long, short = 'z')]
    ligate_zwj: bool,

    /// Options for determining output style.
    #[command(flatten, next_help_heading = "Style")]
    style_flags: StyleFlags,

    /// Options for selecting the operating mode.
    #[command(flatten, next_help_heading = "Modes")]
    mode_flags: ModeFlags,

    #[arg(long, hide = true)]
    #[cfg(debug_assertions)]
    debug: bool,
}

impl Command {
    const fn mode(&self) -> Mode {
        let ModeFlags {
            quenya,
            gondor,
            beleriand,
            /*english,*/
            by_name,
        } = self.mode_flags;

        if let Some(mode) = by_name {
            mode
        } else if quenya {
            Mode::Quenya
        } else if gondor {
            Mode::Gondor
        } else if beleriand {
            Mode::Beleriand
        /*} else if english {
            Mode::English*/
        } else {
            Mode::DEFAULT
        }
    }

    const fn runner(&self) -> Runner {
        let mut runner = Runner::new(self.mode());
        // runner.alt_a = self.style_flags.alt_a;
        // runner.alt_rince = self.style_flags.alt_rince;
        runner.ligate_short = self.ligate_short;
        runner.ligate_zwj = self.ligate_zwj;
        runner.nuquerna = !self.style_flags.no_nuquernar;
        // runner.vowels = self.style_flags.vowels;
        runner
    }
}


fn main() {
    let command: Command = clap::Parser::parse();
    let runner = command.runner();

    #[cfg(debug_assertions)]
    if command.debug {
        dbg!(command);
        exit(0);
    }

    if command.text.is_empty() {
        for line in stdin().lock().lines() {
            if let Ok(text) = line {
                let conv: String = runner.convert(text);

                println!("{}", conv);
            }
        }
    } else {
        let text: String = command.text.join(" ");
        let conv: String = runner.convert(text);

        print!("{}", conv);
        exit(stdout().write(b"\n").is_err() as i32);
    }
}
