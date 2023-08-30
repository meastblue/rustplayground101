pub struct Display;

impl Display {
    pub fn draw_welcome() {
        println!(
            r"
    __  _____    _   __________  ______    _   __
   / / / /   |  / | / / ____/  |/  /   |  / | / /
  / /_/ / /| | /  |/ / / __/ /|_/ / /| | /  |/ / 
 / __  / ___ |/ /|  / /_/ / /  / / ___ |/ /|  /  
/_/ /_/_/  |_/_/ |_/\____/_/  /_/_/  |_/_/ |_/   
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

    pub fn draw(turn: &i32) {
        let draw = Self.draw_turn(turn);
        println!("{}", draw);
    }

    pub fn display_win() {
        println!(
            r"
                                  _ 
   __  ______  __  __   _      __(_)___ 
  / / / / __ \/ / / /  | | /| / / / __ \
 / /_/ / /_/ / /_/ /   | |/ |/ / / / / /
 \__, /\____/\__,_/    |__/|__/_/_/ /_/ 
/____/                                  		
		"
        )
    }

    pub fn display_loose(word: &str) {
        println!(
            r"
                         __                    
   __  ______  __  __   / /___  ____  ________ 
  / / / / __ \/ / / /  / / __ \/ __ \/ ___/ _ \
 / /_/ / /_/ / /_/ /  / / /_/ / /_/ (__  )  __/
 \__, /\____/\__,_/  /_/\____/\____/____/\___/ 
/____/        
		"
        );

        println!("The word was: {}", &word);
    }

    fn draw_turn(&self, turn: &i32) -> &str {
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
		   ____
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
            6 => {
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
