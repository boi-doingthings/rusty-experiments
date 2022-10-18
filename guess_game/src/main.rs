use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::env;
fn main() {  
    env::set_var("RUST_BACKTRACE", "full");      
    let secret_number = rand::thread_rng().gen_range(-100..=100);
    
    // println!("The secret number is :{}",secret_number);

    loop {
        println!("Make a Guess!!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failure");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Less!!") ,
            Ordering::Greater=> println!("Too More!!"),
            Ordering::Equal => {
                println!("You WIN!! :P");
                break;
            }
        };    
    }

}
