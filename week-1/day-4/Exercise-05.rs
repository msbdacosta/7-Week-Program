// Exercise 5: Challenge -- Number Guessing Game

use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("🎯 Guess the number between 1 and 10!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number.");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too low!");
        } else if guess > secret_number {
            println!("Too high!");
        } else {
            println!("✅ You guessed it! The secret number was {}!", secret_number);
            break;
        }
    }
}
