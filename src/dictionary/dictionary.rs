use rand::Rng;
use std::fs;
use std::io::{self, BufRead, Error, ErrorKind};

#[derive(Debug)]
pub struct Dictionary {
    word: String,
}

impl Dictionary {
    pub fn new() -> Result<Self, Error> {
        let words = Self::pick_word()?;
        let rng = rand::thread_rng().gen_range(0..words.len());
        let word = words[rng].clone();

        Ok(Dictionary { word })
    }

    fn pick_word() -> Result<Vec<String>, Error> {
        let file = fs::File::open("static/words.txt")
            .map_err(|e| Error::new(ErrorKind::NotFound, format!("Error opening file: {}", e)))?;
        let reader = io::BufReader::new(file);
        let content = reader.lines().collect::<Result<Vec<String>, Error>>()?;

        if content.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Empty word list"));
        }

        Ok(content)
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }
}
