use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("Hello, world!");
    println!("Welcome to the game");
    let mut random_number = rand::thread_rng().gen_range(1,30);
   loop{
    println!("Enter a number:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    let guess: u32 = guess.trim().parse().expect("Invalid input");

    println!("You guessed: {}", guess);

   
  

    match guess.cmp(&random_number) {
        Ordering::Less => println!("{}", "too small".red()),
        Ordering::Equal=> {

            println!("{}", "You Win!".green());
            break;
        },
        Ordering::Greater=> println!("{}", "too high!".red()),

    }
   }
}
