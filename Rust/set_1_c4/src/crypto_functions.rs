use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Clone)]
pub struct MaxScoreBytes {
    pub hexinput: String,
    pub maxscorebytes: Vec<u8>,
    pub maxscore: u32,
}

//Decodes a hexidecimal string into a set of bytes
fn decode_byte_hex(hex_input: &str) -> Vec<u8> {
    let mut bytes3: Vec<u8> = Vec::with_capacity(hex_input.len() / 2);
    for i in 0..(hex_input.len() / 2) {
        let decimal = u8::from_str_radix(&hex_input[2 * i..2 * i + 2], 16).unwrap();
        bytes3.push(decimal);
    }

    bytes3
}

//Gets the xor of a hexidecimal and a decimal byte representation.
pub fn get_xor_hex_bytes(hex_input: &str, charcode: u8) -> Vec<u8> {
    let mut xor_output = Vec::new();

    let bytes = decode_byte_hex(hex_input);
    for byte in &bytes {
        let xor_val = byte ^ charcode;
        xor_output.push(xor_val);
    }
    xor_output
}

//Gets the number of occurences a set of chracter codes have in a set of bytes.
fn get_score(xor_bytes: Vec<u8>) -> u32 {
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
pub fn get_max_score_bytes(hex_input: &str) -> MaxScoreBytes {
    let mut max_score_charcode = 0;
    let mut max_score = 0;

    for i in 0..128 {
        let xor_bytes = get_xor_hex_bytes(hex_input, i);
        let score = get_score(xor_bytes);
        if score > max_score {
            max_score = score;
            max_score_charcode = i;
        }
    }

    MaxScoreBytes {
        hexinput: hex_input.to_string(),
        maxscorebytes: get_xor_hex_bytes(hex_input, max_score_charcode),
        maxscore: max_score,
    }
}

//Gets the string that is encrypted from a list of strings.
pub fn get_encrypted_string(max_score_bytes: Vec<MaxScoreBytes>) -> MaxScoreBytes {
    let mut max_max_score_bytes = MaxScoreBytes {
        hexinput: "".to_string(),
        maxscorebytes: vec![0],
        maxscore: 0,
    };
    for max_score_byte in max_score_bytes {
        if max_score_byte.maxscore > max_max_score_bytes.maxscore {
            max_max_score_bytes.hexinput = max_score_byte.hexinput;
            max_max_score_bytes.maxscorebytes = max_score_byte.maxscorebytes;
            max_max_score_bytes.maxscore = max_score_byte.maxscore;
        }
    }

    max_max_score_bytes
}

//Returns a vector of strings from a file
pub fn read_file_to_string_vec(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    let buffer = BufReader::new(f);
    buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
