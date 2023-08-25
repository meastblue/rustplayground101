use crate::dictionary;
use crate::hangman;
pub struct App;

impl App {
    pub fn new() {
        Self::run();
    }

    fn run() -> Result<(), std::io::Error> {
        hangman::display::Display::draw_welcome();
        println!("Welcome to hangman game");
        dictionary::dictionary::Dictionary::new()?;

        Ok(())
    }
}
