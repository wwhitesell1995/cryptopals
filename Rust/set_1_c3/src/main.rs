mod crypto_functions;
use std::str;

fn main() {
    output_decrypted_string();
}

fn output_decrypted_string() {
    let hex_input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let max_score_bytes = crypto_functions::get_max_score_bytes(hex_input);
    let mut decrypted_string = "".to_string();
    for byte in max_score_bytes {
        decrypted_string.push_str(str::from_utf8(&vec![byte]).unwrap());
    }

    println!("The decoded string is: {0}", decrypted_string);
}
