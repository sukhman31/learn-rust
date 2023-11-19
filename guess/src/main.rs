use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Input your guess:");

        let mut guess= String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");

        let guess: u32 = match guess.trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
            Ordering::Greater => println!("Too big")
        };
    }

    // println!("The secret number is {secret_number}");

    // println!("You guessed : {guess}");
}
