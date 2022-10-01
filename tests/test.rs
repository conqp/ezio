use ezio::{FileReadable, FileWritable};
use std::path::Path;

const BYTES: [u8; 512] = [
    165, 110, 47, 167, 159, 132, 224, 116, 210, 238, 23, 111, 246, 246, 255, 180, 156, 137, 58,
    125, 3, 96, 133, 24, 122, 134, 232, 118, 140, 5, 91, 105, 66, 128, 113, 0, 200, 31, 103, 194,
    209, 25, 168, 51, 184, 171, 68, 51, 169, 180, 195, 13, 115, 148, 137, 40, 11, 41, 176, 223, 70,
    224, 153, 38, 152, 0, 142, 0, 233, 72, 62, 180, 249, 54, 236, 188, 181, 148, 227, 55, 219, 212,
    77, 109, 184, 139, 116, 111, 216, 210, 254, 132, 108, 14, 255, 64, 217, 238, 41, 190, 151, 207,
    22, 117, 47, 42, 131, 67, 3, 2, 143, 106, 122, 254, 165, 75, 27, 117, 205, 209, 171, 197, 122,
    35, 170, 242, 88, 183, 230, 33, 134, 242, 107, 111, 92, 22, 64, 63, 140, 65, 177, 214, 192,
    204, 66, 187, 11, 12, 173, 144, 156, 53, 211, 48, 170, 9, 191, 144, 106, 1, 57, 89, 86, 49, 9,
    41, 11, 192, 222, 187, 180, 59, 169, 189, 105, 9, 53, 17, 252, 163, 1, 251, 45, 1, 230, 168,
    25, 147, 108, 193, 94, 151, 244, 216, 119, 50, 43, 53, 52, 249, 136, 26, 103, 47, 219, 179,
    239, 49, 212, 48, 243, 230, 115, 137, 57, 192, 145, 22, 252, 248, 212, 49, 222, 192, 36, 233,
    185, 24, 241, 109, 240, 219, 233, 218, 35, 143, 188, 27, 96, 143, 186, 146, 229, 178, 253, 111,
    196, 158, 217, 244, 10, 15, 105, 111, 157, 125, 17, 186, 9, 79, 56, 13, 113, 160, 100, 36, 193,
    48, 146, 198, 207, 33, 17, 87, 178, 72, 154, 56, 86, 147, 220, 86, 79, 238, 28, 202, 190, 11,
    28, 121, 140, 231, 228, 100, 153, 88, 249, 65, 247, 141, 99, 252, 67, 226, 13, 189, 44, 196,
    118, 133, 201, 32, 108, 238, 170, 226, 44, 191, 58, 128, 100, 137, 30, 67, 98, 117, 62, 128,
    114, 51, 187, 136, 211, 255, 42, 229, 17, 86, 71, 25, 191, 110, 222, 244, 227, 39, 210, 232,
    165, 254, 225, 70, 200, 19, 198, 179, 76, 97, 242, 99, 147, 222, 141, 50, 163, 224, 97, 30,
    252, 139, 27, 221, 104, 190, 186, 126, 110, 227, 178, 247, 69, 115, 8, 154, 54, 166, 226, 61,
    148, 146, 89, 11, 228, 93, 10, 96, 84, 182, 101, 150, 32, 10, 157, 159, 79, 141, 212, 54, 111,
    169, 171, 96, 135, 132, 154, 162, 246, 196, 25, 32, 224, 35, 144, 152, 224, 192, 5, 35, 201,
    47, 172, 45, 99, 88, 234, 254, 127, 179, 230, 221, 109, 183, 233, 102, 216, 8, 38, 36, 253,
    176, 127, 210, 92, 103, 154, 126, 98, 185, 113, 20, 62, 44, 116, 14, 101, 183, 16, 192, 241, 5,
    17, 152, 175, 39, 131, 1, 90, 204, 225, 31, 114, 112, 168, 58, 187, 29, 11, 172, 158, 139, 53,
    50, 94, 130, 77, 105, 40, 119, 89, 237, 130, 204, 156, 78, 66, 49, 111, 19, 74, 77, 215, 225,
];
const TEXT_FILE: &str = "tests/textfile.txt";
const BINARY_FILE: &str = "tests/bytesfile.bin";
const TEXT: &str = "Hello, world!";

#[test]
fn read_string() {
    assert_eq!(
        "Hello, wolrd!",
        String::read(Path::new(TEXT_FILE)).unwrap_or("".to_string())
    );
}

#[test]
fn read_string_nsf() {
    assert_eq!(
        "-",
        String::read(Path::new("tests/no-such-file")).unwrap_or("-".to_string())
    );
}

#[test]
fn read_bytes() {
    assert_eq!(
        Vec::from(BYTES),
        Vec::read(Path::new(BINARY_FILE)).unwrap_or(Vec::new())
    );
}

#[test]
fn write_string() {
    let mut file = Path::new("tests/string.wfile");
    assert_eq!((), String::from(TEXT).write(&mut file).unwrap());
    assert_eq!(TEXT, String::read(&mut file).unwrap());
}

#[test]
fn write_bytes_array() {
    let mut file = Path::new("tests/bytes-array.wfile");
    assert_eq!((), BYTES.write(&mut file).unwrap());
    assert_eq!(Vec::from(BYTES), Vec::read(&mut file).unwrap());
}

#[test]
fn write_bytes_vec() {
    let mut file = Path::new("tests/bytes-vec.wfile");
    assert_eq!((), Vec::from(BYTES).write(&mut file).unwrap());
    assert_eq!(Vec::from(BYTES), Vec::read(&mut file).unwrap());
}
