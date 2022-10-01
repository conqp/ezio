# Easy file I/O
This crate provides the traits `FileReadable` and `FileWritable` to allow common data types be directly read / written from / to `std::path::Path` objects.

## Reading
### Text
```rust
use std::path::Path;

use ezio::FileReadable;

fn main() {
    let text_file = Path::new("my_file.txt");

    match String::read(text_file) {
        Ok(text) => println!("My text file contains:\n{}", text),
        Err(msg) => eprintln!("Error reading file: {}", msg),
    }
}
```
### Bytes
```rust
use std::path::Path;

use ezio::FileReadable;

fn main() {
    let binary_file = Path::new("my_file.bin");

    match Vec::read(binary_file) {
        Ok(text) => println!("My text file contains:\n{}", text),
        Err(msg) => eprintln!("Error reading file: {}", msg),
    }
}
```
## Writing
### Text
```rust
use std::path::Path;

use ezio::FileWritable;

fn main() {
    let text = String::from("Hello, world!");
    let text_file = Path::new("my_file.txt");

    match text.write(text_file) {
        Err(msg) => eprintln!("Error writing file: {}", msg),
        _ => println!("Text written."),
    }
}
```
### Bytes
You can write bytes as `[u8]`
```rust
use std::path::Path;

use ezio::FileWritable;

fn main() {
    let bytes: [u8; 13] = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33];
    let binary_file = Path::new("my_file.bin");

    match bytes.write(binary_file) {
        Err(msg) => eprintln!("Error writing file: {}", msg),
        _ => println!("Text written."),
    }
}
```
or as `Vec`:
```rust
use std::path::Path;

use ezio::FileWritable;

fn main() {
    let bytes: Vec<u8> = Vec::from([72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]);
    let binary_file = Path::new("my_file.bin");

    match bytes.write(binary_file) {
        Err(msg) => eprintln!("Error writing file: {}", msg),
        _ => println!("Text written."),
    }
}
```
