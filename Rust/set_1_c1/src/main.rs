mod hex_to_base64;

fn main() {
    let hex_input="49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64string=hex_to_base64::encode_bytes(hex_to_base64::decode_hex(&hex_input));
    output_base64string(&base64string)
}

fn output_base64string(base64string: &str)
{
    let base64result="SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    
    if base64result==base64string.to_string()
    {
      println!("The string {0} matches with the result.", base64string);    
    }
    else
    {
      println!("Error! Base 64 string {0} didn't match with result!", base64string);
    }
}

