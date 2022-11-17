#[macro_use]
extern crate clap;

mod bin_mode;
mod bin_test;

use std::{io::{BufRead, stdin, stdout, Write}, process::exit};
use bin_mode::*;
use tengwar::{TranscriberSettings, VowelStyle};


#[derive(Args, Debug)]
struct ModeFlags {
    /// Transliterate in the Classical Quenya Mode (default).
    ///
    /// Independent Tengwar represent consonant sounds, with vowels being
    ///     represented by a Tehta placed above either the preceding consonant
    ///     or a "carrier" mark.
    ///
    /// This mode is typically used for Quenya.
    #[arg(long, short = 'Q')]
    #[arg(group = "mode")]
    quenya: bool,

    /// Transliterate in the Sindarin Mode of Gondor.
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

    /// Transliterate in the Sindarin Mode of Beleriand.
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
    #[arg(long, short = 'E')]
    #[arg(group = "mode")]
    english: bool,*/

    /*/// Set a mode by language.
    #[arg(long = "lang", short = 'L', value_name = "LANG")]
    #[arg(group = "mode", value_enum, ignore_case = true)]
    language: Option<Language>,*/

    /// Set a mode by name.
    #[arg(long = "mode", short = 'M', value_name = "MODE")]
    #[arg(group = "mode", value_enum, ignore_case = true)]
    by_name: Option<Mode>,
}


#[derive(Args, Debug)]
struct StyleFlags {
    /// Use the alternate "yanta" tehta for A-vowels.
    ///
    /// The alternate form is simpler and much quicker to write by hand than the
    ///     default tri-dot, and may be preferred when typesetting text intended
    ///     to be handwritten.
    #[arg(long, short = 'a')]
    alt_a: bool,

    /// Use a more ornate "sa-rincë" for final sibilants.
    ///
    /// The basic rincë is a small hook attached to the bottom of a tengwa. This
    ///     option enables the use of a larger "curl" version attached to the
    ///     right side.
    ///
    /// The alternate version may only be applied to the final tengwa in a word,
    ///     and only to certain tengwar.
    #[arg(long, short = 'r')]
    alt_rince: bool,

    /// Set behavior for long vowel tehtar.
    ///
    /// Generally, a long vowel may be indicated by
    ///     (1) writing the tehta twice,
    ///     (2) writing the tehta on an extended carrier, or
    ///     (3) writing a vertical line below the base tengwa.
    ///
    /// The first method has two possible implementations:
    ///     (1a) writing the same codepoint twice, and
    ///     (1b) using a dedicated codepoint for a doubled tehta.
    /// The extended carrier forms a ZWJ ligature with many tengwar, becoming a
    ///     vertical line below, making the third method a special case of the
    ///     second method (combined with `--ligate-zwj`).
    ///
    /// Support for these methods and implementations varies between fonts, so
    ///     all are provided as possibilities. However, certain tehtar are not
    ///     suitable for doubling, and so will always use the separate extended
    ///     carrier, regardless of this setting.
    #[arg(long = "long", short = 'l', value_name = "STYLE")]
    #[arg(group = "tehtar", value_enum, ignore_case = true)]
    #[arg(default_value_t = VowelStyle::DEFAULT)]
    vowels: VowelStyle,

    /// Do not use inverted "nuquerna" variants.
    ///
    /// Some tengwar typically occupy the center space above them, where a vowel
    ///     diacritic would be placed. When one of these tengwar needs to have a
    ///     vowel, it is often inverted to make room; This option prevents that,
    ///     as some typefaces can handle it well.
    #[arg(long = "no-nuquernar", short = 'n')]
    #[arg(action = clap::ArgAction::SetFalse)]
    nuquerna: bool,
}


/// Transliterate text into J.R.R. Tolkien's Tengwar.
///
/// Since the Tengwar are simply a writing system, and not a full language,
///     there are various "modes" that can be used for transliteration. The
///     default is the Classical Mode, mainly used for Quenya, but others are
///     available for selection by command line options.
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

    /// Use the ligated short carrier where applicable.
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

    /// Show the parsed input settings and immediately exit.
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
            /*language,*/
            by_name,
        } = self.mode_flags;

        if let Some(mode) = by_name {
            mode
        /*} else if let Some(lang) = language {
            lang.mode()*/
        } else if quenya {
            Mode::Classical
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
        Runner::new(self.mode(), self.settings())
    }

    const fn settings(&self) -> TranscriberSettings {
        TranscriberSettings {
            alt_a: self.style_flags.alt_a,
            alt_rince: self.style_flags.alt_rince,
            ligate_short: self.ligate_short,
            ligate_zwj: self.ligate_zwj,
            nuquerna: self.style_flags.nuquerna,
            vowels: self.style_flags.vowels,
        }
    }
}


fn main() {
    let command: Command = clap::Parser::parse();
    let runner = command.runner();

    #[cfg(debug_assertions)]
    if command.debug {
        let mode = command.mode();
        let settings = command.settings();
        dbg!(command, mode, settings);
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
