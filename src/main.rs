use std::cmp::Ordering;
use std::io;
use std::ops::{Add, Range};

use rand::Rng;

fn main() {
    let random_number: i32 = rand::thread_rng().gen_range(Range { start: 1, end: 100 });
    println!("Guess a number between {} and {}!\nPlease input your guess.", 1, 100);
    let mut guesses: i32 = 0;
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read your input.");
        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        guesses = guesses.add(1);
        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("Your guess {} is too small!\nPlease input your guess.", guess);
                continue;
            }
            Ordering::Equal => {
                println!("You won! Guesses needed: {}", guesses);
                break;
            }
            Ordering::Greater => {
                println!("Your guess {} is too big!\nPlease input your guess.", guess);
                continue;
            }
        }
    }
}