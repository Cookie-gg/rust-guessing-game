use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // create new String instance

        io::stdin()
            .read_line(&mut guess) // give reference to guess (pointer?)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() { // parse returns Result type (Ok, Err)
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // println!("The secret number is: {secret_number}");

        match guess.cmp(&secret_number) { // cmp returns Ordering type (Less, Greater, Equal)
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
