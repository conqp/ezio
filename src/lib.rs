use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;

pub trait FileReadable {
    fn from_reader(reader: &mut impl Read) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(file: &Path) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::from_reader(&mut open(file)?)
    }
}

pub fn open(file: &Path) -> Result<BufReader<File>, Error> {
    Ok(BufReader::new(File::open(file)?))
}

impl FileReadable for String {
    fn from_reader(reader: &mut impl Read) -> Result<Self, Error> {
        let mut content = Self::new();

        match reader.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(code) => Err(code),
        }
    }
}

impl FileReadable for Vec<u8> {
    fn from_reader(reader: &mut impl Read) -> Result<Self, Error> {
        let mut content = Self::new();

        match reader.read_to_end(&mut content) {
            Ok(_) => Ok(content),
            Err(code) => Err(code),
        }
    }
}
