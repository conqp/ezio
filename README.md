# Easy file I/O
This crate provides a trait `FileReadable` to allow common data types be directly read from `std::path::Path` objects.

## Reading and writing text
```rust
use std::path::Path;

use ezio::FileReadable;

fn main() {
    let text_file = Path::new("my_file.txt");

    match String::read(text_file) {
        Ok(text) => {
            println!("My text file contains:\n{}", text);

            let new_file = Path::new("new_file.txt");
            text.write(new_file);
        }
        Err(msg) => eprintln!("Error reading file: {}", msg),
    }
}
```

## Reading and writing bytes
```rust
use std::path::Path;

use ezio::FileReadable;

fn main() {
    let binary_file = Path::new("my_file.bin");

    match Vec::read(binary_file) {
        Ok(bytes) => {
            /*
                Do something with bytes.
                bytes: Vec<u8>
            */
            
            let new_file = Path::new("new_file.bin");
            bytes.write(new_file);
        }
        Err(msg) => eprintln!("Error reading file: {}", msg),
    }
}
```