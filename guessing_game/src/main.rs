use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game");

    loop {
        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess (or type quit to exit):");

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        if guess.trim() == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("The secret number is: {}", secret_number);
    }
}