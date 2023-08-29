use std::usize;

pub struct Display;

impl Display {
    pub fn draw_welcome() {
        println!(
            r"
			|\     /|(  ___  )( (    /|(  ____ \(       )(  ___  )( (    /|
			| )   ( || (   ) ||  \  ( || (    \/| () () || (   ) ||  \  ( |
			| (___) || (___) ||   \ | || |      | || || || (___) ||   \ | |
			|  ___  ||  ___  || (\ \) || | ____ | |(_)| ||  ___  || (\ \) |
			| (   ) || (   ) || | \   || | \_  )| |   | || (   ) || | \   |
			| )   ( || )   ( || )  \  || (___) || )   ( || )   ( || )  \  |
			|/     \||/     \||/    )_)(_______)|/     \||/     \||/    )_)
		"
        );

        Self::draw_rules();
    }

    fn draw_rules() {
        println!(
            "Welcome to Hangman Game!\n\
			------------------------\n\
			Hangman is a word guessing game.\n\
			Here are the rules:\n\
			1. A secret word is chosen by the computer.\n\
			2. You need to guess the letters in the word.\n\
			3. You have a total of 8 incorrect guesses before the hangman is complete.\n\
			4. Each incorrect guess adds a part to the hangman drawing.\n\
			5. If you correctly guess all the letters in the word before using all 8 guesses, you win!\n\
			6. If the hangman is fully drawn before you guess the word, you lose.\n\
			Have fun and good luck!\n"
        );
    }

    pub fn draw(turn: &usize) {
        let draw = Self.draw_turn(turn);
        println!("{}", draw);
    }

    fn draw_turn(&self, turn: &usize) -> &str {
        let draw = match turn {
            0 => {
                r" 
			____
			|    |      
			|    o      
			|   /|\     
			|    |
			|   / \
		   _|_
		  |   |______
		  |          |
		  |__________|"
            }
            1 => {
                r"
			____
			|    |      
			|    o      
			|   /|\     
			|    |
			|    
		   _|_
		  |   |______
		  |          |
		  |__________|"
            }
            2 => {
                "
			  ____
			 |    |      
			 |    o      
			 |    |
			 |    |
			 |     
			_|_
		   |   |______
		   |          |
		   |__________|"
            }
            3 => {
                "
			  ____
			 |    |      
			 |    o      
			 |    
			 |    
			 |     
			_|_
		   |   |______
		   |          |
		   |__________|"
            }
            4 => {
                "
			  ____
			 |    |      
			 |          
			 |    
			 |    
			 |     
			_|_
		   |   |______
		   |          |
		   |__________|"
            }
            5 => {
                "
			
			 |          
			 |          
			 |    
			 |    
			 |     
			_|_
		   |   |______
		   |          |
		   |__________|"
            }
            7 => {
                "
			_ _
		   |   |______
		   |          |
		   |__________|"
            }
            _ => "",
        };

        draw
    }
}
