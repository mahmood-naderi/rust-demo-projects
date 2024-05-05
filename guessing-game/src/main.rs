use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("This is the guessing number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("{}", secret_number);
            
    loop {
        println!("Enter your guess ...");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read your input!");

        println!("you guessed {}", guess);

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "You guessed way lower ...".red()),
            Ordering::Greater => println!("{}", "You guessed way higher ...".red()),
            Ordering::Equal => {
                println!("{}", "You have guessed the SECRET NUMBER!!!".green());
                break;
            },
        }
    }

}