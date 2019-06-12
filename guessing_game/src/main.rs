use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_random_number() -> u8 {
    rand::thread_rng().gen_range(1,11)
}

fn get_guess() -> u8 {
    println!("Please input your guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u8 = guess.trim().parse().expect("Please type a number");
    guess
}

fn guessing_game() {
    println!("\nOkay, I have a number!");
    let random_number = get_random_number();
    loop {
        let guess = get_guess();
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => break,
        }
    }
    println!("Congratulations! The number was {}", random_number);
}

fn get_is_playing_another() -> bool {
    println!("Would you like to play again? (Y/n)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");
    response.to_lowercase().starts_with("y")
}

fn main() {
    let mut is_playing_another = true;
    while is_playing_another {
        guessing_game();
        is_playing_another = get_is_playing_another();
    }
    println!("\nThanks for playing!");
}