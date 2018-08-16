//Decodes a hexidecimal string into a set of 3 bytes
pub fn decode_hex(hex_input: &str) -> Vec<u32> {
    let mut bytes3: Vec<u32> = Vec::with_capacity(hex_input.len() / 6);
    for i in 0..(hex_input.len() / 6) {
        let decimal = u32::from_str_radix(&hex_input[6 * i..6 * i + 6], 16).unwrap();
        bytes3.push(decimal);
    }

    bytes3
}

//Gets the xor of two hexidecimal values
pub fn get_xor_hex(hex_input: &str, xor_input: &str) -> String {
    let mut xor_output = "".to_owned();

    if hex_input.len() == xor_input.len() {
        let decimal_hex_input = decode_hex(hex_input);
        let decimal_xor_input = decode_hex(xor_input);
        for i in 0..decimal_hex_input.len() {
            let xor_val = decimal_hex_input[i] ^ decimal_xor_input[i];
            let hex_xor_val = format!("{:x}", xor_val);
            xor_output.push_str(&hex_xor_val);
        }
    } else {
        panic!("Error! Hexidecimal strings must match in length!");
    }

    xor_output
}
