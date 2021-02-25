use itertools::Itertools;
use std::env::args;
use tengwar::{Quenya, Rules};


fn main() {
    println!("{}", Quenya::transcribe(
        args().skip(1).intersperse(String::from(" ")).collect()
    ));
}
