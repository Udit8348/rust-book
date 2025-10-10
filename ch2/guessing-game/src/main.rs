// Rust automatically imports a small set of common items into every program (the "prelude").
// Anything else must be brought into scope explicitly with `use`.
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please enter your guess:");

    // `String` is a growable, UTF-8 encoded text type.
    // `new()` is an "associated function" — called on a type rather than an instance.
    let mut guess = String::new();

    /*
        `read_line` returns a `Result<T, E>` enum used for error handling:

            enum Result<T, E> {
                Ok(T),   // success value
                Err(E),  // error value
            }

        `.expect()` extracts the value if it's `Ok`, or crashes with a message if it's `Err`.
        This is fine for simple programs — later you'll use `match` or `?` for safer error handling.
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Shadowing: reuse the guess variable name rather than forcing us to create two unique variables
    let guess: i32 = guess.trim().parse().unwrap();

    // Create a random number generator and produce a random i32 between 1 and 10 (inclusive).
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=10);

    // Compare the guess to the generated number and print the result.
    if random_number != guess {
        println!("Wrong! The number was: {}", random_number);
    } else {
        println!("Right!");
    }
}
