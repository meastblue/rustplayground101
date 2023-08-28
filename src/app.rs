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
        let mut game = game::Game::new(word);
        hangman::display::Display::draw_welcome();

        loop {
            if game.get_state() != game::GameState::Pending {
                break;
            }

            if game.get_turns_left() == 0 {
                game.set_state(game::GameState::Loose);
                break;
            }
        }

        Ok(())
    }
}
