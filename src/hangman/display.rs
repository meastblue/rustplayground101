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
    }

    pub fn draw(turn: &usize) {
        let draw = Self.draw_turn(turn);
        println!("{}", draw);
    }

    fn draw_turn(&self, turn: &usize) -> &str {
        let mut draw = match turn {
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
