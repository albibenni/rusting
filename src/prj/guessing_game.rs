#![allow(dead_code)]

use rand::Rng;

pub fn guess() -> () {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number!");

        let mut guess = String::new();

        std::io::stdin() // alternative to the import and call io:stdin() fucntion
            .read_line(&mut guess)
            .expect("Failed to read line"); // without expect it won't compile - possible error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {secret_number}");
}
