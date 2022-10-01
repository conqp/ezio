# Easy file I/O
This crate provides a trait `FileReadable` to allow common data types be directly read from `use std::path::Path` objects.

## Read a string
Given `~/myfile.txt`:
```rust
use std::path::Path;

use ezio::FileReadable;

main () {

}
match String::read(file) {
    Ok(text) => {
        let extension = match file.extension() {
            Some(suffix) => suffix.to_str().unwrap_or(""),
            None => "",
        };

        match file.to_str() {
            Some(filename) => {
                println!("`{}`:", filename);
                println!(
                    "```{}\n{}```",
                    LANGUAGES.get(extension).unwrap_or(&""),
                    text
                );
            }
            None => eprintln!("Could not extract file name."),
        }
    }
    Err(code) => eprintln!("Error reading file: {}", code),
}