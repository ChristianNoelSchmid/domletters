//! Example implementation of dom_letters library
//!
//! Christian Schmid
//! June 2021

use std::{io, io::Read};

use domletters::dom_letters;

fn main() {
    match read_string() {
        Ok(v) => println!("dominant letter count: {}", dom_letters(v)),
        Err(_) => print_usage(),
    }
}

/// Reads the string given by stdin. If there is a read
/// `Error`, prints usage.
fn read_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    Ok(buffer)
}

/// Prints the usage of the application, should
/// a user-error arise. Then exits with code `1`.
fn print_usage() -> ! {
    println!("Usage domletters: domletters <string_argument>");
    std::process::exit(1);
}
