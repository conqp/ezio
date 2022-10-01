# Easy file I/O
This crate provides a trait `FileReadable` to allow common data types be directly read from `use std::path::Path` objects.

## Reading text
Given `~/myfile.txt`:
```rust
use std::path::Path;

use ezio::FileReadable;

main () {
    let text_file = Path::new("/my/file.txt");

    match String::read(text_file) {
        Ok(text) => {
            // text: String
            println!("My text file contains:\n{}", text)
        }
        Err(msg) => eprintln!("Error reading file: {}", msg),
    }
}

## Reading bytes
Given `~/myfile.txt`:
```rust
use std::path::Path;

use ezio::FileReadable;

main () {
    let binary_file = Path::new("/my/file.bin");

    match String::read(binary_file) {
        Ok(bytes) => {  // bytes: Vec<u8>
            // Do something with bytes.
        }
        Err(msg) => eprintln!("Error reading file: {}", msg),
    }
}
