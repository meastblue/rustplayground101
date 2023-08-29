use std::f32::consts::E;

use super::guess::Guess;

#[derive(PartialEq, Clone)]
enum GameState {
    Pending,
    Loose,
    Win,
}

#[derive(Clone)]
pub struct Game {
    state: GameState,
    letters: Vec<char>,
    found_letters: Vec<char>,
    used_letters: Vec<char>,
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
                    return Some(c);
                }

                None
            })
            .collect::<Vec<char>>();

        let game = Game {
            state: GameState::Pending,
            letters,
            found_letters: vec![],
            used_letters: vec![],
            turns_left,
        };

        game
    }

    pub fn play(&self, word: &str) {
        println!("Here the word:");
        self.display_word(word);
        println!("You have {} turns left", self.turns_left);
        println!("Make a guess");

        match Guess::read_guess() {
            Ok(guess) => println!("{}", guess),
            Err(err) => println!("{:?}", err),
        };
    }

    fn display_word(&self, word: &str) {
        let w = word
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 || i == word.len() - 1 {
                    return c;
                }

                if self.found_letters.contains(&c) {
                    return c;
                }

                return '_';
            })
            .collect::<Vec<char>>();

        for c in w {
            print!("{} ", c);
        }
    }

    fn is_good(&mut self, guess: &char) {
        if !self.letters.contains(&guess) {
            self.is_wrong();
        }

    }

    fn is_wrong(&mut self) {
        if self.turns_left == 0 {
            self.is_over();
        }

        --self.turns_left;
    }

    pub fn is_over(&self) -> bool {
        let mut is_over = false;

        if self.state != GameState::Pending {
            is_over = true;
        }

        if self.turns_left == 0 {
            is_over = true;
        }

        is_over
    }

    pub fn get_turns_left(&self) -> i32 {
        self.turns_left
    }
}
