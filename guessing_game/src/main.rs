use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number : i32 = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // like variables, references are immutable by default.
            //      Hence, you need to write &mut guess rather than &guess to make it mutable
            .expect("Failed to read line");

        // guess -> using the value directly
        // &guess -> using the value immutably (read only)
        // mut& guess -> using the value mutably (rw access)

        let guess: i32 = guess.trim().parse() // this feature is called shadowing
            .expect("Please type a number!");
        // Allows a variable to be redeclared with the same name, even with a new type or immutability.
        // In Rust, variables are immutable by default, which means that when a value is
        // assigned, the value won't change. To make variables mutable, we add mut

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

// By default, Rust has a set of items defined in the standard library that
// it brings into the scope of every program. This set is called the prelude

// If a type you want to use isn’t in the prelude, you have to bring that type into scope
// explicitly with a use statement. Using the std::io library provides you with a number
// of useful features, including the ability to accept user input.
