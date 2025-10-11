// Rust automatically imports a small set of common items into every program (the "prelude").
// Anything else must be brought into scope explicitly with `use`.
use std::cmp::Ordering;
use std::io;
use rand::Rng;


/**
        `read_line` returns a `Result<T, E>` enum used for error handling:

        enum Result<T, E> {
            Ok(T),   // success value
            Err(E),  // error value
        }
*/

fn main() {
    // generate a random number
    let rng = rand::thread_rng().gen_range(1..=10);

    
    loop {
        // new() is an example of an associated function that can be called on a type, rather than an instance
        let mut guess = String::new();

        println!("please input your guess");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // we can route a result using a match statement
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        // use a match to determine action
        match guess.cmp(&rng) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;}
        }
    }    
}
