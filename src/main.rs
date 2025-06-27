use std::io; // For reading input
use rand::Rng; // For generating random numbers
use std::cmp::Ordering; // For comparing values

fn main() {
    println!("🔢 Welcome to Guess the Number!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! 📉"),
            Ordering::Greater => println!("Too big! 📈"),
            Ordering::Equal => {
                println!("🎉 You win! The number was {}.", secret_number);
                break;
            }
        }
    }
}
