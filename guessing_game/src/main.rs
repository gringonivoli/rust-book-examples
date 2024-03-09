use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    Ok(loop {
        println!("Please input your guess.");

        let mut guess_input = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        // let guess = Guess::new(guess_input.trim().parse()?);
        let guess = Guess::new(match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        });

        // let guess: u32 = match guess_input.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        println!("You guessed: {}", guess.value());
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break ();
            }
        }
    })
}
