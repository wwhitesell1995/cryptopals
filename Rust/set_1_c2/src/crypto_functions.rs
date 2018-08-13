//Gets the xor of two hexidecimal values
pub fn get_xor_hex(hex_input: &str, xor_string: &str) -> String
{
    let mut xor_output="".to_owned();
    
    if hex_input.len()==xor_string.len()
    {
        for i in 0..hex_input.len()
        {
            let decimal_hex_input=u64::from_str_radix(&hex_input[i..i+1], 16).unwrap();
            let decimal_xor_string=u64::from_str_radix(&xor_string[i..i+1], 16).unwrap();
            let xor_val=decimal_hex_input^decimal_xor_string;
            let hex_xor_val=format!("{:x}", xor_val);
            xor_output.push_str(&hex_xor_val);
        }
    }
    else
    {
        panic!("Error! Hexidecimal strings must match!");
    }

    xor_output
}
