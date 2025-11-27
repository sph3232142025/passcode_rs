
use std::io::{self, Write};

const RPASSCODE: &str = "zaccy254#";

fn main() {
    print!("ENTER PASSCODE FOR ZACCY DREAMER PROGRAM: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let entered = input.trim_end_matches(&['\n', '\r', ' '][..]).trim();

    if entered == RPASSCODE {
        println!("ACCESS GRANTED.WELCOME, ZACCYDREAMER!");
    } else {
        println!("WRONG PASSCODE YOU FOOL,FU*CK OFF!");
        std::process::exit(1);
    }
}