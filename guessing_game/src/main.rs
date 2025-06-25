use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("the secret number is: {}", secret_number);

    loop {
        println!("Input your guess: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guess correctly");
                break;
            } 
        }
    }
}