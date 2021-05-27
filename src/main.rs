extern crate num_traits;

mod lz77;

use std::fs::File;
use std::io::Read;

/// Read file content as vector of bytes.
fn file_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect("no file found");
    let metadata = std::fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return buffer;
}

/// CLI application.
/// 
/// When called with a parameter, it will open that file and perform the LZ77 compression (nothing gets saved).
/// 
/// When called with no parameters, it will output a dry run  on screen.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        let input = file_bytes(filename);
        let encoded = lz77::encode::encode::<u8, u8>(&input);
        println!("File is {} bytes long.", input.len());
        println!("Encoding size: {} codes.", encoded.len());
    } else {
        let input = "12341234abcabcabcabcXXXXXXXXXXXXX";
        let encoded = lz77::encode::from_string::<u8>(&input);
        let decoded = lz77::decode::to_string(&encoded);
        println!("Usage: lz77 [file]");
        println!("If file provided, it encodes it and prints the compressed size.");
        println!("No file given, so printing an example run now.");
        println!("{} is the input.", input);
        println!("{} is obtained by encoding and decoding.", &decoded);
        println!("Input has {} characters, encoded in {} codes.", input.len(), encoded.len());
        println!("Input size: {} bytes.", std::mem::size_of_val(input));
    }
}
