use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=200);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read the line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match random_number.cmp(&guess) {
            Ordering::Greater => println!("Too low."),
            Ordering::Less => println!("Too big."),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
