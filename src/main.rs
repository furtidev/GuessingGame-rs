use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {


    println!("🤔 Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1..101);

    loop {
        println!("⌨️  Input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("❌ {}", "Your number is too small!".red()),
            Ordering::Greater => println!("❌ {}", "Your number is too big!".red()),
            Ordering::Equal => {
                println!("✔️  {}", "You Win!".green()); 
                break;
            },
        }
    }

}