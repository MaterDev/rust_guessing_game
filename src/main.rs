// This application needs 3 things
// User input
// Comparing Numbers
// Generate random numbers

use rand::Rng;
use std::{cmp::Ordering, io}; // Destructuring imports

// ! main() is the entrypoint for the program
fn main() {
    // Print message to console, similar to console.log() in Javascript.
    println!("Guessing Number Game");

    // ! Generate a secret number.
    // rand::thread_rng - thread-local random number generator, seeded by the system
    let secret_number: i8 = rand::thread_rng().gen_range(1..=100); // `=100` will include the number 100 as a possibility

    // println!("Secret Number: {secret_number}");

    // Infinite loop to ask for guesses.
    // ! With an infinite loop and a simple println statement, the program ran 135,943 times
    loop {
        println!("Guess a number <= 100");

        // ! Create empty string to store guess
        // `mut` makes this variable mutable. (by default, variables in rust are immutable)
        let mut guess: String = String::new();

        /*
        ! Read a line of input from the user into the `guess` string
            `&mut guess` is a reference to the mutable `guess` string
         */
        io::stdin() // A handle to the standard input stream of a process.
            .read_line(&mut guess) // Locks this handle and reads a line of input, appending it to the specified buffer.
            .expect("Failed to read line"); // Error handling, will display specified message

        // println!("Your guess: {guess}");

        /*

        ! This will convert the `guess` (string) into an 8-bit integer
            - `trim()` will remove leading and trailing whitespace
            - `parse()` converts data type
        */
        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num, // Parse will retrun `ok` if it succeeds, in which the new type will be stored in `guess`

            /*
               When continue is encountered, the current iteration is terminated, returning control to the loop head, typically continuing with the next iteration.
            */
            Err(_) => continue,
        };

        // println!("Guess: {guess}");

        // ! Copmpare the guess with the secret number
        // `.cmp()` - This method returns an [Ordering] between self and other.
        match guess.cmp(&secret_number) {
            // Check if guess is too big/small or equal
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                // ! If equal will break out of loop and end game!
                println!("You Win!");
                break;
            }
        }
    }
}
