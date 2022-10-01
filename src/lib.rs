use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;

pub fn read(file: &Path) -> Result<String, Error> {
    let fh = File::open(file)?;
    let mut buf_reader = BufReader::new(fh);
    let mut content = String::new();

    match buf_reader.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(code) => Err(code),
    }
}
