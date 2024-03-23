/*
 * Crate is a collection of Rust source code files
 * rand crate is a library crate
 */

use rand::Rng;
/**
 * Prelude - set of items defined in the standard library that it brings to the scope of every program
 * Standard library
 */
use std::{cmp::Ordering, io};
/**
 * fn(): declares a new function; the parenthesis (), indicates there are no parameters
 * Starts and ends the function with {}
 */
fn main() {
    // println! is macro
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        /*
         * Variable to store user input
         * let creates variable
         * variables are immutable by default
         * The syntax ::new is associated function of the string type
         * new function creates a new empty string
         * & indicates that the argument is a reference, gives a way to let multiple part of the code access one piece of data without needing to copy the data into memory multiple times
         */
        let mut guess = String::new();

        /*
         * read_line returns Result value which is enumeration,the type that can be in one of multiple possible states. Each of them is called variant.
         * Result Variation - OK and Err
         */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadows the previous guess variable with new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
         * {} is placeholder
         */
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
