use rust_challenges::challenges::*;

use std::io;

fn main() {
    loop {
        println!("Enter 'q' to quit.");
        println!("Select a challenge to run:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => palindrome_integer::print_solve(),
            "q" | "Q" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid selection."),
        }
    }
}
