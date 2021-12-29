use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secred_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input something");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You Guessed {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secred_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }
    }
}
