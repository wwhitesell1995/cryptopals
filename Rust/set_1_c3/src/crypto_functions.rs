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
pub fn get_max_score_bytes(hex_input: &str) -> Vec<u8> {
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

    get_xor_hex_bytes(hex_input, max_score_charcode)
}
