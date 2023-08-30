use crate::dictionary::dictionary::Dictionary;
use crate::hangman::display::Display;
use crate::hangman::game;

pub struct App;

impl App {
    pub fn new() {
        Self::run();
    }

    pub fn run() {
        let dictionary = match Dictionary::new() {
            Ok(d) => d,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        let word = dictionary.get_word();
        let mut game = game::Game::new(&word);

        Display::draw_welcome();

        while !game.is_over() {
            match game.play(&word) {
                Some(_) => {
                    Display::display_win();
                    return;
                }
                None => continue,
            }
        }

        Display::display_loose(&word);
    }
}
