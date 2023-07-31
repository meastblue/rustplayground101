use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut try_count = 10;
    println!("Guess the number!");
    println!("The secret number is between 1 and 100.");
    println!("You have {} tries to guess the number.", try_count);

    loop {
        let user_input: i32 = match get_user_input().trim().parse() {
            Ok(num) => num,
            Err(err) => panic!("Error: {}", err),
        };

        if check_guess(secret_number, user_input) {
            println!("You win!");
            break;
        }

        if try_count == 0 {
            println!("You lose!");
            break;
        }

        try_count -= 1;
    }
}

fn get_user_input() -> String {
    let mut guess = String::new();

    println!("Please input your guess.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

fn check_guess(guess: i32, input: i32) -> bool {
    if input > guess {
        println!("Too big!");
        return false;
    }

    if input < guess {
        println!("Too small!");
        return false;
    }

    true
}