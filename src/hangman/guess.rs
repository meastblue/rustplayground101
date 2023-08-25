use std::io;

#[derive(Debug)]
enum GuessError {
    InputTooLong,
    ParseError,
}
pub struct Guess;

impl Guess {
    pub fn read_guess() -> io::Result<char> {
        loop {
            let mut g = String::new();

            println!("What is your letter?");
            io::stdin().read_line(&mut g)?;

            match Self::check_guess(&g) {
                Ok(c) => return Ok(c),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    fn check_guess(guess: &str) -> Result<char, GuessError> {
        let trim_guess = guess.trim();

        if trim_guess.len() != 1 {
            return Err(GuessError::InputTooLong);
        }

        let guess_char = trim_guess.chars().next().ok_or(GuessError::ParseError)?;

        Ok(guess_char)
    }
}
