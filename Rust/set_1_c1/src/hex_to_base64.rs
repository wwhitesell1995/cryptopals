//Created my own hexidecimal to base 64 converter. I know there are external crates that do this as well I just wanted to create my own :)
pub fn hex_to_b64(hex_input: &str) -> String
{
    let mut base64string: String="".to_owned();
    let base64chars=vec!["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z",
                        "a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z",
                        "0","1","2","3","4","5","6","7","8","9","+","/"];

    let mut binary="".to_owned();
    for c in hex_input.chars()
    {
        let decimal=u64::from_str_radix(&c.to_string(), 16).unwrap();
        binary.push_str(&format!("{:04b}", decimal));
    }

    for i in 0..(binary.len()/6) {
        let index=usize::from_str_radix(&binary[(6*i)..(6*(i+1))], 2).unwrap();
        base64string.push_str(&base64chars[index]);
    }
    
    base64string
}