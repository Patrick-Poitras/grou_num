#[cfg(test)]
pub mod test_radix_convert {
    use grou_num::radix_convert::*;
    use grou_num::grou::Grou;
    #[test]
    fn test_base10_to_binary() {
        let base10_num_inputs :[Vec<u8>;7] = [
            vec![1,2,3,4,5,6,7,8,9,0],
            vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5],
            vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0],
            vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5],
            vec![1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0],
            vec![0],
            vec![],
        ];
        let grou_num_outputs:  [Grou; 7] = [
            Grou::from(1234567890),
            Grou::from(123456789012345),
            Grou::from(vec![12345678901234567890]),
            Grou::from(vec![1096246371337559929, 66926]),
            Grou::from(vec![14083847773837265618, 6692605942]),
            Grou::from(vec![0]),
            Grou::from(vec![]),
        ];
        for (ind, number_vec) in base10_num_inputs.iter().enumerate() {
            let result = convert_digits_to_binary(number_vec.as_slice());
            assert_eq!(result, grou_num_outputs[ind]);
        }
    }

    #[test]
    fn test_convert_from_string() {
        let base10_str_inputs :[&str;7] = [
            "1234567890",
            "123456789012345",
            "12345678901234567890",
            "1234567890123456789012345",
            "123456789012345678901234567890",
            "0",
            "",
        ];
        let grou_num_outputs:  [Grou; 7] = [
            Grou::from(1234567890),
            Grou::from(123456789012345),
            Grou::from(vec![12345678901234567890]),
            Grou::from(vec![1096246371337559929, 66926]),
            Grou::from(vec![14083847773837265618, 6692605942]),
            Grou::from(vec![0]),
            Grou::from(vec![]),
        ];

        for (&input, output) in base10_str_inputs.iter().zip(grou_num_outputs) {
            assert_eq!(Grou::from(input), output);
        }

        let base16_str_inputs : [&str; 6] = [
            "0xffffffffffffffffffffffffffffffff",
            "0x0123456789012345678901234567890",
            "0x314159265358979323846fffffffffff",
            "0x0000000000000000000000000000000000000000000000000000f",
            "0",
            "",
        ];

        let grou_num_outputs:  [Grou; 6] = [
            Grou::from(vec![18446744073709551615,18446744073709551615]),
            Grou::from(vec![6230888492328974480, 5124095575331380]),
            Grou::from(vec![2559293633555595263, 3549216002486605715]),
            Grou::from(vec![15]),
            Grou::from(vec![0]),
            Grou::from(vec![]),
        ];

        for (&input, output) in base16_str_inputs.iter().zip(grou_num_outputs) {
            assert_eq!(Grou::from(input), output);
        }
    }
}
