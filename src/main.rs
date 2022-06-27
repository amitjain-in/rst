use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let random = rand::thread_rng().gen_range(1..10);


    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Enter an integer instead");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&random) {
            Ordering::Equal => {
                println!("Correct guess");
                break;
            },
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too big")
        }
    }

}