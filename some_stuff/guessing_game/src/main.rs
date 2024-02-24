use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a random number!!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guessed_number = String::new();
        println!("Input your guess");

        io::stdin()
        .read_line(&mut guessed_number)
        .expect("Something is wrong");

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is: {}", guessed_number);

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is smaller!"),
            Ordering::Greater => println!("Your guess is bigger!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }  
}
