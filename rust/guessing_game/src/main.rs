use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // println! is a macro, not a function.
    // Macros are denotated by the '!' characther.
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please, input your guess.");
        let mut guess = String::new();

        // Call stdin function from the io module.
        io::stdin()
            // Store the input line in guess variable.
            // Argument must be a reference of a mutable variable.
            .read_line(&mut guess) 
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, input a number!");
                continue;
            }
        };

        // {} is a placeholder
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
