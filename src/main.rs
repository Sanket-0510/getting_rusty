use std::io;
use rand::Rng
fn main() {
    println!("Hello, world!");
    println!("welcome to the game");
    println!("enter the number");
    let mut guess  = String::new();
    io::stdin().read_line(&mut guess).expect("failed to take the input");
    println!("you guess {}", guess);

}
