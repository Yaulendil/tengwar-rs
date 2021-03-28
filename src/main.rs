use itertools::Itertools;
use std::{io::{stdin, stdout, BufRead, Write}, process::exit};
use tengwar::{Quenya, ToTengwar};


fn main() {
    let args = std::env::args().skip(1);

    if args.len() > 0 {
        print!("{}", args.intersperse(String::from(" "))
            .collect::<String>()
            .to_tengwar::<Quenya>());
        exit(stdout().write(b"\n").is_err() as i32);
    } else {
        let stream = stdin();
        let mut lines = stream.lock().lines();

        while let Some(Ok(line)) = lines.next() {
            println!("{}", line.to_tengwar::<Quenya>());
        }
    }
}
