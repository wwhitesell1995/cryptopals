//Created my own hexidecimal to base 64 converter. I know there are external crates that do this as well I just wanted to create my own :)
pub fn hex_to_b64(hex_input: &str) -> String
{
    let mut base64string: String="".to_string();
    let base64chars=vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z",
                        "a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z",
                        "0","1","2","3","4","5","6","7","8","9","+","/"];

    let mut bytes: Vec<u32> = Vec::with_capacity(hex_input.len() / 6);
    for i in 0..(hex_input.len()/6)
    {
        println!("{:?}", &hex_input[6*i .. 6*i+6]);
        let decimal=u32::from_str_radix(&hex_input[6*i .. 6*i+6], 16).unwrap();
        bytes.push(decimal);
    }

    for byte3 in &bytes  {
        let div_index3=byte3/262144;
        let div_index2=(byte3%262144)/4096;
        let div_index1=(byte3%4096)/64;
        let rem_index=byte3%64;
        
        if div_index3>0
        {
            base64string.push_str(&base64chars[div_index3 as usize]);
        }
        
        if div_index2>0
        {
            base64string.push_str(&base64chars[div_index2 as usize]);
        }
        
        if div_index1>0
        {
            base64string.push_str(&base64chars[div_index1 as usize]);
        }
        
        base64string.push_str(&base64chars[rem_index as usize]);
    }
    
    base64string
}