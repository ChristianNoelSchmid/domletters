use std::{io, io::Read};

use prog::dom_letters;

fn main() {
    match read_string() {
        Ok(v) => println!("dominant letter count: {}", dom_letters(v)),
        Err(_) => print_usage(),
    }
}

fn read_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn print_usage() -> ! {
    println!("Usage domletters: domletters <string_argument>");
    std::process::exit(1);
}
