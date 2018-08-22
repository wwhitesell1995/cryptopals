mod crypto_functions;

fn main() {
    let repeating_xor_key_value =
        crypto_functions::get_repeating_xor_key_value_from_file("set_1_c6_input.txt");
    println!(
        "The repeating xor key is: {}\n",
        repeating_xor_key_value.key
    );
    println!(
        "The repeating xor value is:\n {}",
        repeating_xor_key_value.value
    );
}
