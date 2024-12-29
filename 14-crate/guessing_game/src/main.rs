use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };

        let guess = Guess::new(guess);

        println!("you guessed : {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too Small!!"),
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
    
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        
        if value < 1 { panic!("Guess value must be greater than or equal to 1, got {value}") }
        
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}