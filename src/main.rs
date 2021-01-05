use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    loop{
        println!("Guess the number!");
        let mut guess = String::new();
        let number = rand::thread_rng().gen_range(1, 1000);
        println!("The secret number is {}", number);
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess:u16 = guess.trim().parse().expect("Failed!");
        if guess == number {
            println!("Correct!");
        } else {
            println!("Wrong!");
            println!("something!");
        }
    }
}
