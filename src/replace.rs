use std::{
    fs::File,
    io::{BufRead, BufReader, Result, Write},
};

pub struct Replace {
    src: String,
    dist: String,
    target: String,
    word: String,
}

impl Replace {
    pub fn new(src: String, dist: String, target: String, word: String) -> Self {
        Replace {
            src,
            dist,
            target,
            word,
        }
    }

    pub fn find_and_replace(&mut self) -> Result<()> {
        let src = File::open(&self.src)?;
        let mut dist = File::create(&self.dist)?;
        let reader = BufReader::new(src);

        for line in reader.lines() {
            let l = line?;
            let buf = self.process_line(l)?;

            dist.write_all(buf.as_bytes())?;
        }

        Ok(())
    }

    fn process_line(&self, line: String) -> Result<String> {
        let target = format!("{} ", &self.target);
        let word = format!("{} ", &self.word);

        if !line.contains(&target) {
            return Ok(line.clone());
        }

        let l = line.replace(&target, &word);
        let l = l.replace(&target.to_lowercase(), &word.to_lowercase());

        Ok(l)
    }
}
