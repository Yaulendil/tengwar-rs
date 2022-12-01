//! Tests for executable binary.
#![cfg(test)]

use std::iter::once;
use crate::Command;


const ARG_Q: &str = "eleni sílar";
const ARG_S: &str = "giliath sílar";


/// Run the CLI self-check.
#[test]
fn test_cli() {
    <Command as clap::CommandFactory>::command().debug_assert();
}


/// Test all the various ways to specify a transcription mode.
#[test]
fn test_modes() {
    //  Default mode.
    run([ARG_Q]);

    //  Dedicated per-mode options.
    run(["--quenya", ARG_Q]);
    run(["--gondor", ARG_S]);
    run(["--beleriand", ARG_S]);
    run(["-Q", ARG_Q]);
    run(["-G", ARG_S]);
    run(["-B", ARG_S]);

    //  Mode option, separate.
    run(["--mode", "classical", ARG_Q]);
    run(["--mode", "Classical", ARG_Q]);
    run(["--mode", "CLASSICAL", ARG_Q]);
    run(["--mode", "c", ARG_Q]);
    run(["--mode", "C", ARG_Q]);
    run(["--mode", "gondor", ARG_S]);
    run(["--mode", "G", ARG_S]);
    run(["--mode", "g", ARG_S]);
    run(["--mode", "beleriand", ARG_S]);
    run(["--mode", "B", ARG_S]);
    run(["--mode", "b", ARG_S]);
    //  Short.
    run(["-M", "classical", ARG_Q]);
    run(["-M", "c", ARG_Q]);
    run(["-M", "C", ARG_Q]);
    run(["-M", "gondor", ARG_S]);
    run(["-M", "G", ARG_S]);
    run(["-M", "g", ARG_S]);
    run(["-M", "beleriand", ARG_S]);
    run(["-M", "B", ARG_S]);
    run(["-M", "b", ARG_S]);

    //  Mode option, joined.
    run(["--mode=classical", ARG_Q]);
    run(["--mode=Classical", ARG_Q]);
    run(["--mode=CLASSICAL", ARG_Q]);
    run(["--mode=c", ARG_Q]);
    run(["--mode=C", ARG_Q]);
    run(["--mode=gondor", ARG_S]);
    run(["--mode=G", ARG_S]);
    run(["--mode=g", ARG_S]);
    run(["--mode=beleriand", ARG_S]);
    run(["--mode=B", ARG_S]);
    run(["--mode=b", ARG_S]);
    //  Short (equal).
    run(["-M=classical", ARG_Q]);
    run(["-M=Classical", ARG_Q]);
    run(["-M=c", ARG_Q]);
    run(["-M=C", ARG_Q]);
    run(["-M=gondor", ARG_S]);
    run(["-M=G", ARG_S]);
    run(["-M=g", ARG_S]);
    run(["-M=beleriand", ARG_S]);
    run(["-M=B", ARG_S]);
    run(["-M=b", ARG_S]);
    //  Short (concat).
    run(["-Mclassical", ARG_Q]);
    run(["-MClassical", ARG_Q]);
    run(["-Mc", ARG_Q]);
    run(["-MC", ARG_Q]);
    run(["-Mgondor", ARG_S]);
    run(["-Mg", ARG_S]);
    run(["-MG", ARG_S]);
    run(["-Mbeleriand", ARG_S]);
    run(["-Mb", ARG_S]);
    run(["-MB", ARG_S]);
}


/// Test some of the ways NOT to specify a transcription mode.
#[test]
fn test_modes_invalid() {
    //  Mode option, without mode.
    run_err(["--mode", "--", ARG_Q]);
    run_err(["-M", "--", ARG_Q]);

    //  Conflicting mode options.
    run_err(["-QG", ARG_S]);
    run_err(["-QB", ARG_S]);
    run_err(["-GB", ARG_S]);
    run_err(["-QGB", ARG_S]);
    run_err(["-Q", "-Mc", ARG_Q]);
    run_err(["-Q", "-Mg", ARG_Q]);
    run_err(["-Q", "-Mb", ARG_Q]);
    run_err(["-G", "-Mc", ARG_S]);
    run_err(["-G", "-Mg", ARG_S]);
    run_err(["-G", "-Mb", ARG_S]);
    run_err(["-B", "-Mc", ARG_S]);
    run_err(["-B", "-Mg", ARG_S]);
    run_err(["-B", "-Mb", ARG_S]);
}


/// Test the transcription style options.
#[test]
fn test_styles() {
    //  Ligature options (long).
    run(["--ligate-all", ARG_Q]);
    run(["--ligate-short", ARG_Q]);
    run(["--ligate-zwj", ARG_Q]);
    run(["--ligate-short", "--ligate-zwj", ARG_Q]);

    //  Ligature options (short).
    run(["-s", ARG_Q]);
    run(["-z", ARG_Q]);
    run(["-sz", ARG_Q]);
    run(["-szzz", ARG_Q]);
    run(["-s", "-z", ARG_Q]);
    run(["-s", "-z", "-zz", ARG_Q]);
    run(["-s", "-z", "-z", "-z", ARG_Q]);

    //  Alternate A-tehta.
    run(["--alt-a", ARG_Q]);
    run(["-a", ARG_Q]);

    //  Alternate Sa-Rincë.
    run(["--alt-rince", ARG_Q]);
    run(["-r", ARG_Q]);

    //  Dots below plain tengwar.
    run(["--dot-plain", ARG_Q]);
    run(["-d", ARG_Q]);

    //  A-tehta elision.
    run(["--elide-a", ARG_Q]);
    run(["-e", ARG_Q]);

    //  Disable Nuquernar.
    run(["--no-nuquernar", ARG_Q]);
    run(["-n", ARG_Q]);

    //  Vowel options (long).
    run(["--long", "separate", ARG_Q]);
    run(["--long", "doubled", ARG_Q]);
    run(["--long", "unique", ARG_Q]);
    run(["--long=separate", ARG_Q]);
    run(["--long=doubled", ARG_Q]);
    run(["--long=unique", ARG_Q]);

    //  Vowel options (short).
    run(["-l", "separate", ARG_Q]);
    run(["-l", "doubled", ARG_Q]);
    run(["-l", "unique", ARG_Q]);
    run(["-l=separate", ARG_Q]);
    run(["-l=doubled", ARG_Q]);
    run(["-l=unique", ARG_Q]);
    run(["-lseparate", ARG_Q]);
    run(["-ldoubled", ARG_Q]);
    run(["-lunique", ARG_Q]);
    run(["-ls", ARG_Q]);
    run(["-ld", ARG_Q]);
    run(["-lu", ARG_Q]);
    run(["-l0", ARG_Q]);
    run(["-l1", ARG_Q]);
    run(["-l2", ARG_Q]);

    //  Vowel options, with no value.
    run_err(["--long=", "doubled", ARG_Q]);
    run_err(["--long", ARG_Q]);
    run_err(["-l=", "doubled", ARG_Q]);
    run_err(["-l", ARG_Q]);
}


fn try_run<'s, I, T>(input: I) -> Result<String, clap::Error> where
    I: IntoIterator<Item=&'s T>,
    T: AsRef<str> + ?Sized + 's,
{
    const BIN: &str = env!("CARGO_BIN_NAME");

    let args: _ = once(BIN).chain(input.into_iter().map(|s| s.as_ref()));
    let exec: Command = clap::Parser::try_parse_from(args)?;
    let text: String = exec.text.join(" ");

    Ok(exec.runner().convert(text))
}

fn run<'s>(input: impl AsRef<[&'s str]>) -> String {
    match try_run(input.as_ref()) {
        Ok(text) => text,
        Err(err) => {
            err.print().expect("Failed to report command failure");
            panic!("Failed to run command: {:?}", input.as_ref());
        }
    }
}

fn run_err<'s>(input: impl AsRef<[&'s str]>) -> clap::Error {
    match try_run(input.as_ref()) {
        Ok(text) => {
            eprintln!(
                "Command should have failed, but succeeded.\
                \n  Args:   {input:?}\
                \n  Output: {output:?}",
                input = input.as_ref(),
                output = text,
            );
            panic!("Command should have failed, but succeeded.");
        }
        Err(err) => err,
    }
}
