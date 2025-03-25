use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    println!("Guess the number!");

    let secreat_number = rand::rng().random_range(1..=101);

    println!("secreat number: {}", secreat_number);

    loop {
        println!(" please input your guess.");

        // in rust varibles are immutable by default(we cant change the value)

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guesses: {}", guess);

        match guess.cmp(&secreat_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "you win!".green());
                break;
            }
        }
    }
}
