use rand::Rng;
use std::fs;
use std::io::{self, BufRead, Error};

pub struct Dictionary {
    word: String,
}

impl Dictionary {
    pub fn new() -> Result<Self, Error> {
        let words = Self::pick_word()?;
        let rng = rand::thread_rng().gen_range(0..=words.len());

        Ok(Dictionary {
            word: words[rng].clone(),
        })
    }

    fn pick_word() -> Result<Vec<String>, Error> {
        println!("IN");
        let file = fs::File::open("../../static/words.txt")?;
        let reader = io::BufReader::new(file);
        let content = reader
            .lines()
            .map(|line| {
                println!("{:?}", &line);
                line?
            })
            .collect::<Result<String>>();

        println!("{:?}", &content);

        Ok(content)
    }
}
