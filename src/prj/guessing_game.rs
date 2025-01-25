//use std::io; // alternative way to import
pub fn guess() -> () {
    println!("Guess the number!");

    let mut guess = String::new();

    std::io::stdin() // alternative to the import and call io:stdin() fucntion
        .read_line(&mut guess)
        .expect("Failed to read line"); // without expect it won't compile - possible error

    println!("You guessed: {}", guess);
}
