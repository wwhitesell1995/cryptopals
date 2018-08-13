mod crypto_functions;

fn main() {
    output_xorstring()
}

fn output_xorstring()
{
    let hex_input="1c0111001f010100061a024b53535009181c";
    let xor_string="686974207468652062756c6c277320657965";
    let xor_result="746865206b696420646f6e277420706c6179";
    let xor_output=crypto_functions::get_xor_hex(&hex_input, &xor_string);
    
    if xor_result==xor_output
    {
      println!("The string {0} matches with the result.", xor_output);    
    }
    else
    {
      panic!("Error! Result strings didn't match for string {0}!", xor_output);
    }
}

