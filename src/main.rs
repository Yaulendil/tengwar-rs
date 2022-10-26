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

    const fn new(
        quenya: bool,
        gondor: bool,
        beleriand: bool,
        /*english: bool,*/
    ) -> Result<Mode, u32> {
        let n = quenya as u32
            + gondor as u32
            + beleriand as u32
            /*+ english as u32*/;

        if n == 0 {
            Ok(Mode::DEFAULT)
        } else if n > 1 {
            Err(n)
        } else if quenya {
            Ok(Mode::Quenya)
        } else if gondor {
            Ok(Mode::Gondor)
        } else if beleriand {
            Ok(Mode::Beleriand)
        /*} else if english {
            Ok(Mode::English)*/
        } else {
            Err(0)
        }
    }

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
struct ModeArg {
    /// Transliterate in the Classical Mode (default).
    #[arg(long, short)]
    #[arg(group = "mode")]
    quenya: bool,

    /// Transliterate in the Mode of Gondor (experimental).
    #[arg(long, short)]
    #[arg(group = "mode")]
    gondor: bool,

    /// Transliterate in the Mode of Beleriand (experimental).
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
/// selection by command line switches.
#[derive(Debug, Parser)]
#[command(version, max_term_width(100))]
struct Command {
    /// Use zero-width joiners to ligate output.
    #[arg(long, short)]
    ligatures: bool,

    /// Text to be transliterated.
    ///
    /// If this is not provided, Standard Input will be used instead.
    text: Vec<String>,

    #[command(flatten)]
    mode: ModeArg,
}


impl Command {
    const fn mode(&self) -> Result<Mode, u32> {
        Mode::new(
            self.mode.quenya,
            self.mode.gondor,
            self.mode.beleriand,
            /*self.mode.english,*/
        )
    }
}


fn main() {
    let cmd: Command = clap::Parser::parse();

    match cmd.mode() {
        Ok(mode) => {
            let xliterate: fn(String) -> String = mode.rules(cmd.ligatures);

            if cmd.text.is_empty() {
                for line in stdin().lock().lines()
                    .filter_map(|x| x.ok())
                    .map(xliterate)
                {
                    println!("{}", line);
                }
            } else {
                print!("{}", xliterate(cmd.text.join(" ")));
                exit(stdout().write(b"\n").is_err() as i32);
            }
        }
        Err(n) => {
            eprintln!("Multiple modes selected.");
            exit(n as i32);
        }
    }
}


#[test]
fn verify_cli() {
    <Command as clap::CommandFactory>::command().debug_assert();
}
