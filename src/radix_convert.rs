use super::grou::Grou;

// The base is the largest number of the form (base)^N <= 2^64
// where N is an integer.

const base_binary : u64 = 1 << 63;
const base_decimal: u64 = 10_000_000_000_000_000_000u64;
const base_hexadecimal: u64 = 0x10000000_00000000;

/// Accepts an ascii string slice and returns a Grou.
impl std::convert::From<&str> for Grou {
    fn from(s : &str) -> Grou {
        if !s.is_ascii() {
            panic!("Input format is invalid.")
        }
        return convert_from_string(s);
    }
}

/// Accepts an ascii string and returns a Grou.
impl std::convert::From<String> for Grou {
    fn from(s : String) -> Grou {
        return Grou::from(&s[..]);
    }
}

/// The function to which the from implementation delegates.
/// It calls convert_to_(format)_string_to_Grou(s), with
/// the format being determined by the prefix of the string, where:
/// 0b : binary
/// 0x : hexadecimal
/// _ : decimal
pub fn convert_from_string(s :&str) -> Grou {
    if s.len() == 0 {
        return Grou::empty(0);
    }
    if s.chars().nth(0).unwrap() == '0' {
        match s.chars().nth(1) {
            Some('x') => {return convert_hexadecimal_string_to_Grou(s);},
            Some('b') => {return convert_binary_string_to_Grou(s);},
            _ => (),
        }
    }

    return convert_decimal_string_to_Grou(s);
}

pub fn convert_hexadecimal_string_to_Grou(s: &str) -> Grou {
    let (prefix, s) = s.split_at(2);
    let base_B_num = string_into_base(s, 15, 16);
    return convert_to_binary(&base_B_num[..], base_hexadecimal);
}

pub fn convert_binary_string_to_Grou(s: &str) -> Grou {
    let (prefix, s) = s.split_at(2);
    let base_B_num = string_into_base(s, 64, 2);
    return convert_to_binary(&base_B_num[..], base_binary);
}

pub fn convert_decimal_string_to_Grou(s : &str) -> Grou {
    let base_B_num = string_into_base(s, 19, 10);
    return convert_to_binary(&base_B_num[..], base_decimal);
}

///Sorts into packets of a given number of 
/// characters. Each packet is prefixed with `prefix`.
fn string_into_packets(s: &str, packet_length: usize) -> Vec<&str> {
    let mut last;
    let mut first = s;
    let mut ret_vec = Vec::<&str>::new();

    let max_string_size = packet_length;
    while first.len() > max_string_size {
        (first, last) = first.split_at(first.len() - max_string_size);
        ret_vec.push(last);
    } 
    if first.len() > 0 {
        ret_vec.push(first);
    }
    return ret_vec;
}
// Converts a string into a proto-number in 
// base B = 10^19.
fn string_into_base(s: &str, packet_length:usize, base: u32) -> Vec<u64> {
    let u = string_into_packets(s, packet_length);
    let mut v = Vec::<u64>::new();
    for proto_s in u {
        v.push(u64::from_str_radix(proto_s, base).unwrap());
    }
    return v;
}

pub fn convert_digits_to_binary(base10_digits: &[u8]) -> Grou {
    let mut base_B_num = Vec::<u64>::new();
    let mut accumulate: u64 = 0;
    let mut multiple: u64 = 1;
    // Reverse to convert to little endian.
    for val in base10_digits.iter().rev() {
        accumulate += multiple * (*val as u64);
        multiple *= 10;

        if multiple == base_decimal {
            multiple = 1;
            base_B_num.push(accumulate);
            accumulate = 0;
        }
    }
    //Get remaining accumulated bits.
    if accumulate > 0 || (base_B_num.len() == 0 && base10_digits.len() > 0) {
        base_B_num.push(accumulate);
    }

    return convert_to_binary(&base_B_num[..], base_decimal);
}

pub fn convert_to_binary(num_in_base: &[u64], base :u64) -> Grou {
    let mut ret_grou = Grou::empty(num_in_base.len());
    for val in num_in_base.iter().rev() {
        ret_grou *= base;
        ret_grou += *val;
    }

    return ret_grou;
}