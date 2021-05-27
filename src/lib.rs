mod lz77;

use num_traits::Bounded;
use std::convert::{TryFrom, TryInto};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Print to browser's console when Wasm has been correctly loaded.
#[wasm_bindgen(start)]
pub fn run() {
    log("Hello from Rust! WASM is now loaded.");
}

/// Data is a slice-like struct. JS will use this to access the
/// underlying data on the heap.
#[wasm_bindgen]
pub struct Data {
    pub address: usize,
    pub length: usize,
}

/// Concrete Wasm implementation of LZ77.
/// It encodes a sequences of bytes, with window size 2^8.
/// Each single code has size 8 + 8 + 8 = 24 bits.
#[wasm_bindgen]
pub fn encode_8(symbols: &[u8]) -> Data {
    return encode::<u8>(symbols);
}

/// Concrete Wasm implementation of LZ77.
/// It encodes a sequences of bytes, with window size 2^16.
/// Each single code has size 8 + 16 + 16 = 40 bits.
#[wasm_bindgen]
pub fn encode_16(symbols: &[u8]) -> Data {
    return encode::<u16>(symbols);
}

/// Concrete Wasm implementation of LZ77.
/// It encodes a sequences of bytes, with window size 2^32.
/// Each single code has size 8 + 32 + 32 = 72 bits.
#[wasm_bindgen]
pub fn encode_32(symbols: &[u8]) -> Data {
    return encode::<u32>(symbols);
}

/// Bridge between pure Rust implementation and Wasm boilerplate.
/// It computes the LZ77 encoding, and returns a slice-like struct Data
/// that Wasm and JS can use to access the Codes on the heap.lz77
/// 
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn encode<U>(symbols: &[u8]) -> Data
where U: TryFrom<usize> + TryInto<usize> + Bounded + Copy + Eq + From<u8> {
    let encoded = lz77::encode::encode::<u8, U>(symbols);
    let encoded_length = encoded.len() * 3;
    let mut encoded_iterator = encoded.iter();
    let mut encoded_symbols = Vec::<U>::with_capacity(encoded_length);
    loop {
        match encoded_iterator.next() {
            Some(code) => {
                encoded_symbols.push(code.offset);
                encoded_symbols.push(code.length);
                encoded_symbols.push(code.literal.into());
            },
            None => {
                break;
            },
        }
    }
    return Data {
        address: unsafe { std::mem::transmute::<*const U, usize>(encoded_symbols.as_ptr()) },
        length: 3 * encoded.len(),
    };
}
