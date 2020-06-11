//mod print;
//mod vars;
//mod types;
//mod loops;
mod pointers_ref;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();
        println!("Guess a number between 1-100");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small. Try again"),
            Ordering::Greater => println!("Too big. Try again"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}
