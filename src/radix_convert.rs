use super::grou::Grou;

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
    panic!("Unimplemented API");
    return Grou::empty(0);
}

pub fn convert_binary_string_to_Grou(s: &str) -> Grou {
    panic!("Unimplemented API");
    return Grou::empty(0);
}

pub fn convert_decimal_string_to_Grou(s : &str) -> Grou {
    let base_B_num = string_into_baseB(s);
    return convert_baseB_to_binary(&base_B_num[..]);
}

fn string_into_packets(s : &str) -> Vec<&str> {
    let mut last;
    let mut first = s;
    let mut ret_vec = Vec::<&str>::new();

    let max_string_size = 19;
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
fn string_into_baseB(s: &str) -> Vec<u64> {
    let u = string_into_packets(s);
    let mut v = Vec::<u64>::new();
    for proto_s in u {
        v.push(proto_s.parse::<u64>().unwrap());
    }
    return v;
}

pub fn convert_digits_to_binary(base10_digits: &[u8]) -> Grou {
    let base_B = 10_000_000_000_000_000_000u64;
    let mut base_B_num = Vec::<u64>::new();
    let mut accumulate: u64 = 0;
    let mut multiple: u64 = 1;
    // Reverse to convert to little endian.
    for val in base10_digits.iter().rev() {
        accumulate += multiple * (*val as u64);
        multiple *= 10;

        if multiple == base_B {
            multiple = 1;
            base_B_num.push(accumulate);
            accumulate = 0;
        }
    }
    //Get remaining accumulated bits.
    if accumulate > 0 || (base_B_num.len() == 0 && base10_digits.len() > 0) {
        base_B_num.push(accumulate);
    }

    return convert_baseB_to_binary(&base_B_num[..]);
}

pub fn convert_baseB_to_binary(base_B_num: &[u64]) -> Grou {
    let base_B = 10_000_000_000_000_000_000u64;
    // Stores the products of the decomposition of the number into
    // base B = 10^19

    let mut ret_grou = Grou::empty(base_B_num.len());
    for val in base_B_num.iter().rev() {
        ret_grou *= base_B;
        ret_grou += *val;
    }

    return ret_grou;
}