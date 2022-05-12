use std::env;

use encryption::encrypt;

fn parse_args(args: &mut env::Args) -> Option<String> {
    args.next();
    match args.next() {
        Some(x) => return Some(String::from(x)),
        None => return None
    }
}

fn main() {
    let input = parse_args(&mut env::args());
    match input {
        Some(i) => {
            let result = encrypt(i);
            println!("{}", result);
        }
        None => println!("usage: cargo run [input]")
    }
}
