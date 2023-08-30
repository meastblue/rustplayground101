use super::display::Display;
use super::guess::Guess;

#[derive(PartialEq, Clone)]
pub enum GameState {
    Pending,
    Loose,
    Win,
}

pub struct Game {
    state: GameState,
    letters: Vec<char>,
    found_letters: Vec<char>,
    turns_left: i32,
}

impl Game {
    pub fn new(word: &str) -> Self {
        let turns_left = 8;
        let letters = word
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                if i != 0 && i != word.len() - 1 {
                    Some(c)
                } else {
                    None
                }
            })
            .collect::<Vec<char>>();

        Game {
            state: GameState::Pending,
            letters,
            found_letters: vec![],
            turns_left,
        }
    }

    pub fn play(&mut self, word: &str) -> Option<GameState> {
        println!("Here the word:");
        self.display_word(word);
        println!("You have {} turns left", self.turns_left);
        println!("Make a guess");

        match Guess::read_guess() {
            Ok(guess) => {
                if !self.is_good(&guess) {
                    Display::draw(&self.turns_left);
                }

                self.found_letters.push(guess);

                if self.is_won() {
                    return Some(GameState::Win);
                }
            }
            Err(err) => println!("{:?}", err),
        }

        None
    }

    fn display_word(&self, word: &str) {
        let w = word
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 || i == word.len() - 1 || self.found_letters.contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect::<Vec<char>>();

        w.iter().for_each(|c| print!("{} ", c));
        println!();
    }

    fn is_good(&mut self, guess: &char) -> bool {
        if !self.letters.contains(&guess) {
            self.is_wrong();
            false
        } else {
            true
        }
    }

    fn is_wrong(&mut self) {
        if self.turns_left == 0 {
            self.is_over();
        } else {
            self.turns_left -= 1;
        }
    }

    pub fn is_over(&mut self) -> bool {
        if self.state != GameState::Pending || self.turns_left == 0 {
            self.state = GameState::Loose;
            true
        } else {
            false
        }
    }

    fn is_won(&self) -> bool {
        self.letters
            .iter()
            .all(|letter| self.found_letters.contains(letter))
    }
}
