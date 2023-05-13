use std::io;
use std::io::Write;

fn main() {
    println!("\nguess the number!");
    
    let mut guess = String::new();

    print!("your guess: ");

    io::stdout().flush().expect("failed to flush");

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    print!("you guessed: {guess}");
}
