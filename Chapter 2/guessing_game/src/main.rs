use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop{
        println!("Please input your guess.");
        let mut guess = String::new(); // Mutable new String.
        io::stdin().    read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => println!("Invalid Number! Remember, the secret number is between 0 and 100."),
        };
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {println!("You win!"); break;}
        };
    }
    println!("The secret number is: {}", secret_number);

}
