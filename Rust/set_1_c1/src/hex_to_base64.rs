//Created my own hexidecimal to base 64 converter. I know there are external crates that do this as well I just wanted to create my own :)

//Decodes a hexidecimal string into a set of 3 bytes
pub fn decode_hex(hex_input: &str)->Vec<u32>
{
    let mut bytes3: Vec<u32> = Vec::with_capacity(hex_input.len() / 6);
    for i in 0..(hex_input.len()/6)
    {
        let decimal=u32::from_str_radix(&hex_input[6*i .. 6*i+6], 16).unwrap();
        bytes3.push(decimal);
    }
    
    bytes3
}

//Encodes a set of 3 bytes and returns the base 64 string
pub fn encode_bytes(bytes3: Vec<u32>) -> String
{
    let mut base64string: String="".to_string();
    let base64chars=vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z",
                        "a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z",
                        "0","1","2","3","4","5","6","7","8","9","+","/"];
                        
    for byte3 in &bytes3  {
        for i in (1..5).rev()
        {  
           let mut curr_byte3_numerator=byte3.clone();
           if i<4 { curr_byte3_numerator=curr_byte3_numerator%64u32.pow(i); }
            
           let div_index=curr_byte3_numerator/64u32.pow(i-1);
          
           
           if div_index>0||i==1 { base64string.push_str(&base64chars[div_index as usize]); }
        }
    }
    
    base64string
}