mod crypto_functions;
use std::str;

fn main() {
    output_decrypted_string();
}

fn output_decrypted_string() {
    let filename = "set_1_c4_input.txt";
    let filehexlines = crypto_functions::read_file_to_string_vec(filename);

    let mut max_score_bytes = Vec::new();

    for hex_input in filehexlines {
        max_score_bytes.push(crypto_functions::get_max_score_bytes(&hex_input));
    }

    let max_max_score_bytes = crypto_functions::get_encrypted_string(max_score_bytes);
    let mut decrypted_string = "".to_string();
    for byte in max_max_score_bytes.maxscorebytes {
        decrypted_string.push_str(str::from_utf8(&vec![byte]).unwrap());
    }

    println!(
        "The encypted string is: {0}\nThe decrypted string is: {1}",
        &max_max_score_bytes.hexinput, decrypted_string
    );
}
