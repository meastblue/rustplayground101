#[derive(PartialEq, Clone)]
pub enum GameState {
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

    pub fn get_state(&self) -> GameState {
        self.state.clone()
    }

    pub fn get_turns_left(&self) -> i32 {
        self.turns_left
    }

    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
    }
}
