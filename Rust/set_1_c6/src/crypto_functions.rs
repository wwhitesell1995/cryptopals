use std::fs::File;
use std::io::prelude::*;
use std::str;

const BASE64CHARS: &'static [&str] = &[
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9", "+", "/",
];

pub struct XorKeyValue {
    pub key: String,
    pub value: String,
}

//Gets the input of a set of xor bytes with a key.
pub fn get_xor_output(input_bytes: Vec<u8>, key: &str) -> String {
    let mut xor_output: Vec<u8> = Vec::new();
    let key_bytes = key.as_bytes();

    for i in 0..input_bytes.len() {
        let xor_val = input_bytes[i] ^ key_bytes[(i % key_bytes.len())];
        xor_output.push(xor_val);
    }

    str::from_utf8(&xor_output).unwrap().to_string()
}

//Gets the hamming distance between a set of bytes.
pub fn get_hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> u32 {
    let bytes_match = bytes1.len() == bytes2.len();
    let bytes_len = match bytes_match {
        true => bytes1.len(),
        false => panic!("Error strings must be of equal length!"),
    };

    let mut hamming_distance = 0;
    for i in 0..bytes_len {
        let mut diff_bits = bytes1[i] ^ bytes2[i];

        while diff_bits > 0 {
            let binary_and_bits = diff_bits & 1;

            if binary_and_bits != 0 {
                hamming_distance += 1;
            }

            diff_bits >>= 1;
        }
    }

    hamming_distance
}

//Converts a base64 string into sets of 3 bytes.
pub fn decode_base64(base64input: &str) -> Vec<u32> {
    let mut byte3 = 0;
    let mut bytes3: Vec<u32> = Vec::new();

    for i in 0..(base64input.len()) {
        let num_byte = i % 4;
        let num_pow = 4 - (num_byte + 1);
        if num_byte == 0 && i > 0 {
            bytes3.push(byte3);
            byte3 = 0;
        }

        let base64index = BASE64CHARS
            .iter()
            .position(|&n| n == &base64input[i..i + 1])
            .unwrap();
        byte3 += 64u32.pow(num_pow as u32) * base64index as u32;
    }
    bytes3.push(byte3);

    bytes3
}

//Turns a base 64 string into bytes
pub fn decoded_base64_to_bytes(base64input: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    let decoded_base64_to_bytes = decode_base64(&base64input);

    for byte3 in &decoded_base64_to_bytes {
        for i in (1..4).rev() {
            let mut curr_byte3_numerator = byte3.clone();
            if i < 3 {
                curr_byte3_numerator %= 256u32.pow(i);
            }

            let byte = curr_byte3_numerator / 256u32.pow(i - 1);
            bytes.push(byte as u8);
        }
    }

    bytes
}

//Formats a base 64 string and returns if it is a valid base64 string.
pub fn format_base64_string(base64input: &str) -> Result<&str, &str> {
    let base64input = base64input.clone().trim_right_matches('=');

    let mut contains = false;
    for i in 0..base64input.len() {
        contains = BASE64CHARS.contains(&&base64input[i..i + 1]);
        if contains == false {
            break;
        }
    }

    match contains {
        true => Ok(base64input),
        false => Err("Error.. An invalid base 64 string was input!"),
    }
}

//Returns a string from a file
fn read_file_to_string(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.replace('\n', "")
}

//Gets the repeating_xor_key from a base 64 encoded string encrypted with a repeating key xor.
pub fn get_repeating_xor_key_value() -> XorKeyValue {
    let base_64_string = read_file_to_string("set_1_c6_input.txt");
    let formatted_base_64_string = format_base64_string(&base_64_string);
    let mut repeating_xor_key = XorKeyValue {
        key: "".to_string(),
        value: "".to_string(),
    };

    if formatted_base_64_string.is_ok() {
        let bytes = decoded_base64_to_bytes(&formatted_base_64_string.unwrap());
        let keysize = get_key_size(&bytes);
        let blocks = get_blocks(&bytes, keysize);
        let repeating_xor_key_char_codes = get_max_score_char_codes(&blocks);
        repeating_xor_key.key = str::from_utf8(&repeating_xor_key_char_codes)
            .unwrap()
            .to_string();
        repeating_xor_key.value = get_xor_output(bytes, &repeating_xor_key.key);
    } else {
        println!("{}", formatted_base_64_string.unwrap());
    }

    repeating_xor_key
}

//Gets the keysize of a set of bytes.
fn get_key_size(bytes: &[u8]) -> u8 {
    let mut min_edit_distance = 99999.0;
    let mut keysize = 0;

    for i in 2..41 {
        let mut distances: Vec<f32> = Vec::new();

        for j in 0..(bytes.len() / i) - 1 {
            let bytes1 = &bytes[i * j..i * j + i];
            let bytes2 = &bytes[i * (j + 1)..i * (j + 1) + i];
            let normalized_distance = get_hamming_distance(&bytes1, &bytes2) as f32;
            distances.push(normalized_distance);
        }

        let avg_edit_distance: f32 =
            (distances.iter().sum::<f32>() / distances.len() as f32) / i as f32;
        if avg_edit_distance < min_edit_distance {
            min_edit_distance = avg_edit_distance;
            keysize = i as u8;
        }
    }

    keysize
}

//Gets the bytes separated into blocks of keysize.
fn get_blocks(bytes: &[u8], keysize: u8) -> Vec<Vec<u8>> {
    let mut blocks: Vec<Vec<u8>> = Vec::new();

    for i in 0..keysize {
        let mut block: Vec<u8> = Vec::new();
        for j in 0..(bytes.len() / keysize as usize) {
            let byte = bytes[i as usize + (j * keysize as usize)];
            block.push(byte);
        }
        blocks.push(block);
    }

    blocks
}

//Gets the xor of a hexidecimal and a decimal byte representation.
pub fn get_xor_bytes(bytes: &[u8], charcode: u8) -> Vec<u8> {
    let mut xor_output = Vec::new();

    for byte in bytes {
        let xor_val = byte ^ charcode;
        xor_output.push(xor_val);
    }
    xor_output
}

//Gets the number of occurences a set of character codes have in a set of bytes.
fn get_score(xor_bytes: &Vec<u8>) -> u32 {
    let mut score = 0;
    let most_used_letters = "ETAOIN SHRDLU";
    for letter in most_used_letters.chars() {
        let uppercase_charcode = letter.to_ascii_uppercase() as u8;
        let lowercase_charcode = letter.to_ascii_lowercase() as u8;

        score += xor_bytes
            .iter()
            .filter(|&n| *n == uppercase_charcode)
            .count() as u32;

        if uppercase_charcode != lowercase_charcode {
            score += xor_bytes
                .iter()
                .filter(|&n| *n == lowercase_charcode)
                .count() as u32;
        }
    }

    score
}

//Gets the character codes for the set of bytes with the largest score.
pub fn get_max_score_char_codes(blocks: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut char_codes: Vec<u8> = Vec::new();

    for block in blocks {
        let mut max_score_charcode = 0;
        let mut max_score = 0;

        for i in 0..128 {
            let xor_bytes = get_xor_bytes(block, i);
            let score = get_score(&xor_bytes);
            if score > max_score {
                max_score = score;
                max_score_charcode = i;
            }
        }

        char_codes.push(max_score_charcode);
    }

    char_codes
}

//Tests the get hamming distance function
#[test]
fn test_get_hamming_distance() {
    let bin_s1 = b"this is a test";
    let bin_s2 = b"wokka wokka!!!";

    assert_eq!(37, get_hamming_distance(bin_s1, bin_s2));
}

//Tests the decode base64 function.
#[test]
fn test_decode_base64() {
    let base64string0 = "AAAA";
    let base64string1 = "AAAB";
    let base64string2 = "////";
    let base64string3 = "Sd+5";
    let base64string4 = format_base64_string("AAAB=");
    let base64string5 = format_base64_string("AAA=B=");

    assert_eq!(vec![0], decode_base64(base64string0));
    assert_eq!(vec![1], decode_base64(base64string1));
    assert_eq!(vec![16777215], decode_base64(base64string2));
    assert_eq!(vec![4841401], decode_base64(base64string3));
    assert!(base64string4.is_ok() && !base64string4.is_err());
    assert_eq!(vec![1], decode_base64(base64string4.unwrap()));
    assert!(base64string5.is_err() && !base64string5.is_ok());
}

//Tests the decoded base64 to bytes function.
#[test]
fn test_decoded_base64_to_bytes() {
    let base64string0 = "////";
    let base64string1 = "AAAAAAAA";
    let base64string2 = "AAABAAAB";
    let base64string3 = "Sd+5";
    let base64string4 = format_base64_string("AAAB=");
    let base64string5 = format_base64_string("AAA&B=");

    assert_eq!(vec![255, 255, 255], decoded_base64_to_bytes(base64string0));
    assert_eq!(
        vec![0, 0, 0, 0, 0, 0],
        decoded_base64_to_bytes(base64string1)
    );
    assert_eq!(
        vec![0, 0, 1, 0, 0, 1],
        decoded_base64_to_bytes(base64string2)
    );
    assert_eq!(vec![73, 223, 185], decoded_base64_to_bytes(base64string3));
    assert!(base64string4.is_ok() && !base64string4.is_err());
    assert_eq!(
        vec![0, 0, 1],
        decoded_base64_to_bytes(base64string4.unwrap())
    );
    assert!(base64string5.is_err() && !base64string5.is_ok());
}
