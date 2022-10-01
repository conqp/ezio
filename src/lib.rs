use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

pub trait FileReadable: Default {
    fn read_from_file(&mut self, fh: &mut File) -> Result<usize, Error>;

    fn read(file: &Path) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut instance = Self::default();

        match instance.read_from_path(file) {
            Ok(_) => Ok(instance),
            Err(err) => Err(err),
        }
    }

    fn read_from_path(&mut self, file: &Path) -> Result<usize, Error> {
        self.read_from_file(&mut File::open(file)?)
    }
}

pub trait FileWritable {
    fn write_to_file(&self, fh: &mut File) -> Result<(), Error>;

    fn write(&self, file: &Path) -> Result<(), Error> {
        self.write_to_file(&mut File::open(file)?)
    }
}

impl FileReadable for String {
    fn read_from_file(&mut self, fh: &mut File) -> Result<usize, Error> {
        fh.read_to_string(self)
    }
}

impl FileReadable for Vec<u8> {
    fn read_from_file(&mut self, fh: &mut File) -> Result<usize, Error> {
        fh.read_to_end(self)
    }
}

impl FileWritable for String {
    fn write_to_file(&self, fh: &mut File) -> Result<(), Error> {
        fh.write_all(self.as_bytes())
    }
}

impl FileWritable for Vec<u8> {
    fn write_to_file(&self, fh: &mut File) -> Result<(), Error> {
        fh.write_all(self)
    }
}
