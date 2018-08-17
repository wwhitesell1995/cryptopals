//Gets the repeating key xor of an input and a key.
pub fn get_xor_bytes(input: &str, key: &str) -> String {
    let mut xor_output = "".to_owned();
    let input_bytes = input.as_bytes();
    let key_bytes = key.as_bytes();

    for i in 0..input_bytes.len() {
        let xor_val = input_bytes[i] ^ key_bytes[(i % key_bytes.len())];
        let hex_xor_val = format!("{:01$x}", xor_val, 2);
        xor_output.push_str(&hex_xor_val);
    }

    xor_output
}
