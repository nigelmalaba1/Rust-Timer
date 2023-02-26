//write a timer app that takes a number of seconds as a command line argument and prints a message after that many seconds have elapsed.
use std::{io::{self, Write}, thread, time::Duration};

fn main() {
    let mut input = String::new();

    print!("Enter the duration of the timer (in seconds): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let duration = Duration::from_secs(input.trim().parse().unwrap());

    println!("Starting timer for {} seconds...", duration.as_secs());

    thread::sleep(duration);

    println!("Timer completed!");
}

