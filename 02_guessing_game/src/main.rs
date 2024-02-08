use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numner!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("the secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");
        let guess_input: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please Enter valid number".red());
                continue;
            }
        };

        println!("You guessed: {}", guess_input);

        match guess_input.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".yellow()),
            Ordering::Greater => println!("{}", "Too Big!".yellow()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
