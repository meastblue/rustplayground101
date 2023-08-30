use std::fs;
use std::io::{BufRead, BufReader, Error};

pub enum Format {
    JSON,
    CSV,
}
pub struct File {
    src_file: String,
    src_format: Format,
    dst_file: String,
    dst_format: Format,
}

impl File {
    pub fn new(src_file: String, src_format: Format, dst_file: String, dst_format: Format) -> Self {
        File {
            src_file,
            src_format,
            dst_file,
            dst_format,
        }
    }

    pub fn read_file(&self) -> Result<(), Error> {
        let file = fs::File::open(&self.src_file)?;
        let reader = BufReader::new(file);
        let content = reader.lines().collect::<Result<Vec<String>, Error>>()?;
        self.convert_file(content);

        Ok(())
    }

    fn convert_file(&self, buffer: Vec<String>) -> serde_json::Value {
        serde_json::json!(buffer)
        // match self.dst_format {
        //     Format::JSON => {
        //         return json!(&buffer);
        //     }
        //     Format::CSV => println!(" "),
        // }
    }
}
