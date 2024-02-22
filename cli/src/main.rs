mod utils;

use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {
    let num_range = 1..=10;
    let secret_number = rand::thread_rng().gen_range(num_range.clone());

    println!(
        "Guess the number! Number is between {} and {}",
        num_range.start(),
        num_range.end(),
    );

    let mut num_guesses = 0;

    loop {
        let guess = utils::get_num_input(Some("Please enter your guess:"));
        num_guesses += 1;
        match guess.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => break,
        }
    }

    println!(
        "Yes, the number was {secret_number}! You got it in {num_guesses} {}",
        match num_guesses {
            1 => "guess",
            _ => "guesses",
        }
    );
}
