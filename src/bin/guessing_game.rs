use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please enter a number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed reading line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Go lower"),
            Ordering::Less => println!("Go higher"),
        }
    }
}
