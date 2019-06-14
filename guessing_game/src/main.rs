use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;


fn flush_stdout() {
    io::stdout().flush().expect("Failed to flush stdout");
}

fn get_random_number() -> u8 {
    rand::thread_rng().gen_range(1,11)
}

fn get_guess() -> u8 {
    print!("Please input your guess: ");
    flush_stdout();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That isn't a number :/"); 
            get_guess()
        }
    };
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
    print!("Would you like to play again? (Y/n): ");
    flush_stdout();

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");
    response.to_lowercase().starts_with("y")
}


fn main() {
    let mut is_playing_another = true;
    println!("Number guessing game. Guess a number between 1 and 10. Let me think of a number");
    while is_playing_another {
        guessing_game();
        is_playing_another = get_is_playing_another();
    }

    println!("\nThanks for playing!");
}
