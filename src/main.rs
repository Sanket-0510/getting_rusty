// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// use colored::*;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    hieght : u32
}
impl  Rectangle {
    fn check(&self, rect: &Rectangle) ->  bool {
         self.width * self.hieght > rect.width*rect.hieght
    }
}

fn main() {
    // println!("Hello, world!");
    // println!("Welcome to the game");
    // let mut random_number = rand::thread_rng().gen_range(1,30);
//    loop{
//     println!("Enter a number:");

//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("Failed to read input");
//     let guess: u32 = guess.trim().parse().expect("Invalid input");

//     println!("You guessed: {}", guess);

   
  

//     match guess.cmp(&random_number) {
//         Ordering::Less => println!("{}", "too small".red()),
//         Ordering::Equal=> {

//             println!("{}", "You Win!".green());
//             break;
//         },
//         Ordering::Greater=> println!("{}", "too high!".red()),

//     }
//    }
 
let rect = Rectangle{
    width: 40,
    hieght:20
};

let rect1 = Rectangle{
    width:16,
    hieght:70
};
   let result = rect.check(&rect1);
    if result {
        println!("circle fits");
    }
    else {
        println!("circle does not fit");
    }

}



fn area(reactangle: &Rectangle)-> u32{
     reactangle.width*reactangle.hieght
}