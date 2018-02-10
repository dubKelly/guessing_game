extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number between 1 and 100...");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Huh?");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Nope, higher..."),
            Ordering::Greater => println!("Nope, lower..."),
            Ordering::Equal => {
                println!("You Win! I was thinking of {}", secret_number);
                break;
            }
        }
    }
}
