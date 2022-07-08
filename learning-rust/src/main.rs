//Let's start with a guessing game



//Picking input from the user

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("\nGuess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut counter_guesses = 0; 

    loop{

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
        //println!("The secret number is: {secret_number}");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => { 
                println!("Too small!") 
                counter_guesses += 1;
            },
            Ordering::Greater => { 
                println!("Too big!")
                counter_guesses += 1;
            },
            Ordering::Equal => {
                counter_guesses += 1;
                println!("You win!");
                println!("You guessed the number in {} guesses!", counter_guesses);
                break;
            }
        } 
    }

}
