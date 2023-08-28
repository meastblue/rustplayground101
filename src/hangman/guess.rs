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
            let mut g = String::new();

            println!("What is your letter?");

            match io::stdin().read_line(&mut g) {
                Ok(_) => {
                    match Self::check_guess(&g) {
                        Ok(c) => return Ok(c),
                        Err(err) => {
                            Self::parse_error_msg(&err);
                        }
                    };
                }
                Err(e) => println!("Please enter a valid input: {} ", e),
            };
        }
    }

    fn check_guess(guess: &str) -> Result<char, GuessError> {
        let trim_guess = guess.trim();

        let g = match trim_guess.chars().next() {
            Some(c) => match c.to_lowercase().next() {
                Some(l) => l,
                None => return Err(GuessError::ParseError),
            },
            None => return Err(GuessError::ParseError),
        };

        if trim_guess.is_empty() {
            return Err(GuessError::WrongInput);
        }

        if trim_guess.len() != 1 {
            return Err(GuessError::InputTooLong);
        }

        if g.is_numeric() {
            return Err(GuessError::WrongInput);
        }

        Ok(g.to_lowercase().next().unwrap())
    }

    fn parse_error_msg(error: &GuessError) {
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
