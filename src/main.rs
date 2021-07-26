use argh::{from_env, FromArgs};
use std::{
    io::{BufRead, stdin, stdout, Write},
    process::exit,
};
use tengwar::{Beleriand, Quenya, Rules, Sindarin};


#[derive(Debug)]
enum Mode {
    Quenya,
    Sindarin,
    Gondor,
    Beleriand,
    /*English,*/
}


impl Mode {
    const DEFAULT: Mode = Mode::Quenya;

    const fn new(
        quenya: bool,
        sindarin: bool,
        gondor: bool,
        beleriand: bool,
        /*english: bool,*/
    ) -> Result<Mode, u32> {
        let n = quenya as u32
            + sindarin as u32
            + gondor as u32
            + beleriand as u32
            /*+ english as u32*/;

        if n == 0 {
            Ok(Mode::DEFAULT)
        } else if n > 1 {
            Err(n)
        } else if quenya {
            Ok(Mode::Quenya)
        } else if sindarin {
            Ok(Mode::Sindarin)
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
                Mode::Sindarin
                | Mode::Gondor => Sindarin::transcribe_with_ligatures,
                Mode::Beleriand => Beleriand::transcribe_with_ligatures,
                /*Mode::English => English::transcribe_with_ligatures,*/
            }
        } else {
            match self {
                Mode::Quenya => Quenya::transcribe,
                Mode::Sindarin
                | Mode::Gondor => Sindarin::transcribe,
                Mode::Beleriand => Beleriand::transcribe,
                /*Mode::English => English::transcribe,*/
            }
        }
    }
}


/**
Transliterate text into the Tengwar of Fëanáro.

Since the Tengwar are simply a writing system, and not a full language, there
are various orthographical modes that can be used for transliteration. The
default is the Classical Mode, mainly used for Quenya, but others are available
for selection by command line switches.

Exit Status:
  0  -- Success.
  1  -- Error while writing output.
  2+ -- Too many mode switches. Status is set to number of switches.
*/ //  NOTE: Block comment is necessary here to properly lay out help text.
#[derive(FromArgs)]
struct Command {
    /// transliterate in the Classical (Quenya) mode (default)
    #[argh(switch, short = 'q')]
    quenya: bool,

    /// transliterate in the Sindarin mode (experimental)
    #[argh(switch, short = 's')]
    sindarin: bool,

    /// transliterate in the Mode of Gondor (experimental)
    /// (identical to `--sindarin`)
    #[argh(switch, short = 'g')]
    gondor: bool,

    /// transliterate in the Mode of Beleriand (experimental)
    #[argh(switch, short = 'b')]
    beleriand: bool,

    /*/// transliterate in the English mode
    #[argh(switch, short = 'e')]
    english: bool,*/

    /// use zero-width joiners to ligate output
    #[argh(switch, short = 'l')]
    ligatures: bool,

    /// text to be transliterated
    #[argh(positional)]
    text: Vec<String>,
}


impl Command {
    const fn mode(&self) -> Result<Mode, u32> {
        Mode::new(
            self.quenya,
            self.sindarin,
            self.gondor,
            self.beleriand,
            /*self.english,*/
        )
    }
}


fn main() {
    let cmd: Command = from_env();

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
