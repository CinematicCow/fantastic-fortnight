#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

extern crate chrono;

use chrono::prelude::*;
use std::io;

fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write;

    print!("{}", user_message);

    io::stdout().flush();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    Ok(buffer.trim_end().to_owned())
}

fn main() {
    let name = input("What is your name? ").expect("Something went wrong!");
    let age = input("What is your age? ")
        .expect("Failed to get age")
        .parse::<i32>()
        .expect("Invalid Age");

    let current_year = Utc::now().year();
    let hundred_year = 100 - age + current_year;

    println!(
        "Hey {}, you will turn a 100 in the year {}",
        name, hundred_year
    );
}
