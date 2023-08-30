use std::io;

#[derive(Debug)]
pub enum GuessError {
    InputTooLong,
    ParseError,
    WrongInput,
}

pub struct Guess;

impl Guess {
    pub fn read_guess() -> Result<char, GuessError> {
        loop {
            let mut input = String::new();

            if let Ok(_) = io::stdin().read_line(&mut input) {
                if let Ok(c) = Self::check_guess(&input) {
                    return Ok(c);
                } else {
                    Self::parse_error_msg(GuessError::ParseError);
                }
            } else {
                println!("Please enter a valid input");
            }
        }
    }

    fn check_guess(guess: &str) -> Result<char, GuessError> {
        let trimmed_guess = guess.trim();

        if trimmed_guess.is_empty() {
            return Err(GuessError::WrongInput);
        }

        let c = match trimmed_guess
            .chars()
            .next()
            .and_then(|c| c.to_lowercase().next())
        {
            Some(l) if l.is_alphabetic() => l,
            _ => return Err(GuessError::ParseError),
        };

        if trimmed_guess.len() != 1 {
            return Err(GuessError::InputTooLong);
        }

        Ok(c)
    }

    fn parse_error_msg(error: GuessError) {
        match error {
            GuessError::WrongInput => println!("The input is not valid"),
            GuessError::InputTooLong => {
                println!("The input is too long, you should insert only one character")
            }
            GuessError::ParseError => {
                println!("An error has occurred during the process: {:?}", error)
            }
        }
    }
}
