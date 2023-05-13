use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("\nguess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is: {secret_number}");

    loop {
        let mut guess = String::new();

        print!("\nyour guess: ");

        io::stdout().flush().expect("failed to flush");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("please enter a number!\n");
                continue;
            },
        };

        print!("you guessed: {guess}\nverdict: ");

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("too small!\n"),
            Ordering::Greater => print!("too big!\n"),
            Ordering::Equal => {
                print!("you win!\n");
                break;
            },
        }
    }
}
