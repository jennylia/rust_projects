extern crate rand; // random library

use std::io;
use std::cmp::Ordering; // Ordering cmp
use rand::Rng; // use you say use rand...

fn main() {
    println!("Guess the number! From a 1 - 100 inclusive");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive lower, exclusive high (1..100)

//    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Strong type.

        // we can shadow... we can re-use the guess name...
        // before we used expect("...") to print an error message
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That wasn't a number, can you just enter a number?");
                continue},
        };

        println!("You guesses: {}", guess);

        // Matching like Erlang?
        // match is a statement
        // Ordering is an emnum
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too much"),
            Ordering::Equal => {
                println!("Win!");
                break;
            }
        }
    }
}
