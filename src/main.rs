use itertools::Itertools;
use tengwar::{Quenya, ToTengwar};


fn main() {
    let string_in: String = std::env::args().skip(1)
        .intersperse(String::from(" "))
        .collect();
    println!("{}", string_in.to_tengwar::<Quenya>());
}
