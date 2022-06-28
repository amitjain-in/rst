use std::cmp::Ordering;
use std::io;
use std::ops::Add;

use rand::Rng;

pub fn guessing_game() {
    println!("Guess the number!");

    let random = rand::thread_rng().gen_range(1..10);
    let mut tries = 0;

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Every programming language should have below feature
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Enter an integer instead");
                tries = tries + 1;
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&random) {
            Ordering::Equal => {
                tries = tries + 1;
                println!("Correct guess after {} tries", tries);
                break;
            }
            Ordering::Less => {
                println!("Too less");
                tries = tries + 1;
            },
            Ordering::Greater => {
                println!("Too big");
                tries = tries + 1;
            }
        }
    }
}