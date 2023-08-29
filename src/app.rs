use std::io::Error;

use crate::dictionary;
use crate::hangman;
use crate::hangman::game;
pub struct App;

impl App {
    pub fn new() {
        Self::run();
    }

    fn run() -> Result<(), Error> {
        let dictionary = dictionary::dictionary::Dictionary::new()?;
        let word = dictionary.get_word();
        let game = game::Game::new(word);
        hangman::display::Display::draw_welcome();

        loop {
            if game.is_over() {
                break;
            }

            game.play(word);
        }

        Ok(())
    }
}
