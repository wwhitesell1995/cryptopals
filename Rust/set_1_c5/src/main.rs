mod crypto_functions;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut key = String::new();
    println!("Repeating-Key XOR Encryptor \n");
    print!("Please insert the text you want encrypted: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();

    print!("\nPlease insert the key you want to encrypt with: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");
    key = key.trim().to_string();

    println!("The input text was: {0}", input);
    println!("The key was: {0}", key);
    println!(
        "The resulting encryption from the input and the key was: \n{0}",
        crypto_functions::get_xor_bytes(&input, &key)
    );
}

//Tests the get xor bytes function
#[test]
fn test_get_xor_bytes() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

    let key = "ICE";

    let result="0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    assert_eq!(result, crypto_functions::get_xor_bytes(input, key))
}
