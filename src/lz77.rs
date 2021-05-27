pub mod encode;
pub mod decode;

/// lZ77 encodes an input into a sequence of Codes.
/// Each Code reports where the match is located in the window (offset),
/// how long it is (length), and first symbol to append afterwards (literal).
/// 
/// T: the type that the input consists of.
/// 
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
#[derive(PartialEq)]
pub struct Code<T, U> {
    pub offset: U,
    pub length: U,
    pub literal: T,
}
