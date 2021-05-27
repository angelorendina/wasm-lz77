use super::Code;
use num_traits::Bounded;
use std::convert::{TryFrom, TryInto};

/// Convenience method that applies LZ77 to a string.
/// 
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn from_string<U>(input: &str) -> Vec<Code<char, U>>
where U: TryInto<usize> + TryFrom<usize> + Bounded + Copy {
    let input_chars: Vec<char> = input.chars().collect();
    return encode::<char, U>(&input_chars);
}

/// The LZ77 compression, the turning input into a sequence of Codes.
/// 
/// T: the type that the input consists of.
/// 
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
pub fn encode<T, U>(input: &[T]) -> Vec<Code<T, U>>
where T: Eq + Copy, U: TryInto<usize> + TryFrom<usize> + Bounded + Copy {
    let mut encoded: Vec<Code<T, U>> = Vec::with_capacity(input.len());
    let mut position: usize = 0;
    while position < input.len() {
        let lookahead = &input[position..];
        let window_start = position.saturating_sub(U::max_value().try_into().ok().unwrap());
        let window = &input[window_start..position];
        let code: Code<T, U> = find_code(&window, &lookahead);
        position += code.length.try_into().ok().unwrap() + 1;
        encoded.push(code);
    }
    return encoded;
}

/// Computes the next Code, given the window and a non-empty lookahead.
/// 
/// T: the type that the input consists of.
/// 
/// U: Any numeric type, whose maximum size describes the size of the window.
/// Essentially, either u8, u16 or u32.
fn find_code<T, U>(window: &[T], lookahead: &[T]) -> Code<T, U>
where T: Eq + Copy, U: TryFrom<usize> {
    let mut lookahead_iterator = lookahead.iter();
    let mut code = Code::<T, usize> {
        offset: 0,
        length: 0,
        literal: *lookahead_iterator.next().unwrap(),
    };
    let mut search_length: usize = 1;
    while search_length < lookahead.len() {
        let search = &lookahead[..search_length];
        let rightmost_match = rfind(window, search);
        if rightmost_match == None {
            break;
        }
        code.offset = window.len() - rightmost_match.unwrap();
        code.length = search_length.into();
        code.literal = *lookahead_iterator.next().unwrap();
        search_length += 1;
    }
    return Code::<T, U> {
        offset: U::try_from(code.offset).ok().unwrap(),
        length: U::try_from(code.length).ok().unwrap(),
        literal: code.literal,
    };
}

/// Returns the position of the rightmost match of search in the given window, if any.
/// 
/// T: the type that the input consists of.
fn rfind<T>(window: &[T], search: &[T]) -> Option<usize>
where T: Eq {
    if search.len() > window.len() {
        return None;
    }
    let mut position: usize = window.len() - search.len();
    'scan: loop {
        let segment = &window[position..(position + search.len())];
        for (segment_next, search_next) in segment.iter().zip(search.iter()) {
            if segment_next != search_next {
                if position == 0 {
                    break 'scan;
                }
                position -= 1;
                continue 'scan;
            }
        }
        return Some::<usize>(position);
    }
    return None;
}

#[cfg(test)]
mod rfind {
    use super::*;

    #[test]
    fn empty_window() {
        let window: [u8; 0] = [];
        let search = [0u8, 1, 2];
        let expected_position: Option<usize> = None;
        let found_position = rfind::<u8>(&window, &search);
        assert_eq!(found_position, expected_position);
    }

    #[test]
    fn short_window() {
        let window = [0u8, 1];
        let search = [0u8, 1, 2];
        let expected_position: Option<usize> = None;
        let found_position = rfind::<u8>(&window, &search);
        assert_eq!(found_position, expected_position);
    }

    #[test]
    fn no_match() {
        let window = [0u8, 1, 2, 3];
        let search = [0u8, 4];
        let expected_position: Option<usize> = None;
        let found_position = rfind::<u8>(&window, &search);
        assert_eq!(found_position, expected_position);
    }

    #[test]
    fn trivial_match() {
        let window = [0u8, 1, 2];
        let search = [0u8, 1, 2];
        let expected_position: Option<usize> = Some(0);
        let found_position = rfind::<u8>(&window, &search);
        assert_eq!(found_position, expected_position);
    }

    #[test]
    fn rightmost_match() {
        let window = [0u8, 1, 2, 3, 0, 1, 2];
        let search = [0u8, 1, 2];
        let expected_position: Option<usize> = Some(4);
        let found_position = rfind::<u8>(&window, &search);
        assert_eq!(found_position, expected_position);
    }
}

#[cfg(test)]
mod find_code {
    use super::*;

    #[test]
    fn no_match() {
        let lookahead = [0u8, 1, 2];
        let window = [10u8, 11, 12];
        let expected_code = Code::<u8, u8> {
            offset: 0,
            length: 0,
            literal: 0,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn ignore_match_at_end() {
        let lookahead = [0u8];
        let window = [0u8, 1];
        let expected_code = Code::<u8, u8> {
            offset: 0,
            length: 0,
            literal: 0,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn trivial_match_with_next() {
        let lookahead = [0u8, 1];
        let window = [0u8];
        let expected_code = Code::<u8, u8> {
            offset: 1,
            length: 1,
            literal: 1,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn match_ignoring_last_is_longest() {
        let lookahead = [0u8, 1, 2, 5];
        let window = [0u8, 1, 0, 1, 2, 5, 0, 1];
        let expected_code = Code::<u8, u8> {
            offset: 6,
            length: 3,
            literal: 5,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn match_is_longest() {
        let lookahead = [0u8, 1, 2, 3, 5];
        let window = [0u8, 1, 0, 1, 2, 0, 1, 2, 3, 4];
        let expected_code = Code::<u8, u8> {
            offset: 5,
            length: 4,
            literal: 5,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn match_ignoring_last_is_rightmost() {
        let lookahead = [0u8, 1];
        let window = [0u8, 1, 2, 0, 1, 0, 1, 2];
        let expected_code = Code::<u8, u8> {
            offset: 3,
            length: 1,
            literal: 1,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }

    #[test]
    fn match_is_rightmost() {
        let lookahead = [0u8, 1, 2, 5];
        let window = [0u8, 1, 2, 0, 1, 0, 1, 2];
        let expected_code = Code::<u8, u8> {
            offset: 3,
            length: 3,
            literal: 5,
        };
        let found_code = find_code::<u8, u8>(&window, &lookahead);
        assert_eq!(found_code, expected_code);
    }
}

#[cfg(test)]
mod encode {
    use super::*;

    #[test]
    fn no_matches() {
        let input = [0u8, 1, 2, 3, 4];
        let expected_encoding = vec![
            Code::<u8, u8> { offset: 0, length: 0, literal: 0 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 1 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 2 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 3 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 4 },
        ];
        let found_encoding = encode::<u8, u8>(&input);
        assert_eq!(found_encoding, expected_encoding);
    }

    #[test]
    fn some_matches() {
        let input = [0u8, 1, 0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3];
        let expected_encoding = vec![
            Code::<u8, u8> { offset: 0, length: 0, literal: 0 },
            Code::<u8, u8> { offset: 0, length: 0, literal: 1 },
            Code::<u8, u8> { offset: 2, length: 2, literal: 2 },
            Code::<u8, u8> { offset: 3, length: 3, literal: 3 },
            Code::<u8, u8> { offset: 4, length: 3, literal: 3 },
        ];
        let found_encoding = encode::<u8, u8>(&input);
        assert_eq!(found_encoding, expected_encoding);
    }
}

#[cfg(test)]
mod from_string {
    use super::*;

    #[test]
    fn empty_string() {
        let input = "";
        let expected_encoding = Vec::<Code<char, u8>>::new();
        let found_encoding = from_string::<u8>(&input);
        assert_eq!(found_encoding, expected_encoding);
    }

    #[test]
    fn string_no_matches() {
        let input = "abcde";
        let expected_encoding = vec![
            Code::<char, u8> { offset: 0, length: 0, literal: 'a' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'b' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'c' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'd' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'e' },
        ];
        let found_encoding = from_string::<u8>(&input);
        assert_eq!(found_encoding, expected_encoding);
    }

    #[test]
    fn string_some_matches() {
        let input = "ababcabcdabcd";
        let expected_encoding = vec![
            Code::<char, u8> { offset: 0, length: 0, literal: 'a' },
            Code::<char, u8> { offset: 0, length: 0, literal: 'b' },
            Code::<char, u8> { offset: 2, length: 2, literal: 'c' },
            Code::<char, u8> { offset: 3, length: 3, literal: 'd' },
            Code::<char, u8> { offset: 4, length: 3, literal: 'd' },
        ];
        let found_encoding = from_string::<u8>(&input);
        assert_eq!(found_encoding, expected_encoding);
    }
}
