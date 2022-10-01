use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

pub trait FileReadable {
    fn read(file: &Path) -> Result<Self, Error>  where Self: Sized;
}

pub trait FileWritable {
    fn write(&self, file: &Path) -> Result<(), Error>;
}

impl FileReadable for String {
    fn read(file: &Path) -> Result<Self, Error> {
        let mut content = Self::new();
        File::open(file)?.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl FileReadable for Vec<u8> {
    fn read(file: &Path) -> Result<Self, Error> {
        let mut content = Self::new();
        File::open(file)?.read_to_end(&mut content)?;
        Ok(content)
    }
}

impl FileWritable for String {
    fn write(&self, file: &Path) -> Result<(), Error> {
        File::open(file)?.write_all(self.as_bytes())
    }
}

impl FileWritable for Vec<u8> {
    fn write(&self, file: &Path) -> Result<(), Error> {
        File::open(file)?.write_all(self)
    }
}
