const BASE64CHARS: &'static [&str] = &[
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9", "+", "/",
    ];
    
//Gets the repeating key xor of an input and a key.
#[allow(dead_code)]
pub fn get_xor_bytes(input: &str, key: &str) -> String {
    let mut xor_output = "".to_owned();
    let input_bytes = input.as_bytes();
    let key_bytes = key.as_bytes();

    for i in 0..input_bytes.len() {
        let xor_val = input_bytes[i] ^ key_bytes[(i % key_bytes.len())];
        let hex_xor_val = format!("{:01$x}", xor_val, 2);
        xor_output.push_str(&hex_xor_val);
    }

    xor_output
}

//Gets the hamming distance between a set of bytes.
pub fn get_hamming_distance(bytes1: &[u8], bytes2: &[u8])->u32
{
    let bytes_match= bytes1.len()==bytes2.len();
    let bytes_len = match bytes_match{
        true => bytes1.len(),
        false => panic!("Error strings must be of equal length!"),
    };
    
    let mut hamming_distance=0;
    for i in 0..bytes_len
    {
        let mut diff_bits=bytes1[i]^bytes2[i];
        
        while diff_bits>0{
            let binary_and_bits= diff_bits & 1;
            
            if binary_and_bits!=0
            {
                hamming_distance+=1;
            }
            
            diff_bits >>= 1;
        }
    }
    
    hamming_distance
}

//Converts a base64 string into sets of 3 bytes.
pub fn decode_base64(base64input: &str)->Vec<u32>
{        
    let mut byte3=0;
    let mut bytes3: Vec<u32>=Vec::new();
    
    for i in 0..(base64input.len())
    {
        let num_byte=i%4;
        let num_pow=4-(num_byte+1);
        if num_byte==0 && i>0
        {
           bytes3.push(byte3);
           byte3=0;
        }

        let base64index=BASE64CHARS.iter().position(|&n| n == &base64input[i..i+1]).unwrap();
        byte3+=64u32.pow(num_pow as u32)*base64index as u32;
    }
    bytes3.push(byte3);
    
    bytes3
}

pub fn decoded_base64_to_bytes(base64input: &str)->Vec<u8>
{    
    let mut bytes: Vec<u8> = Vec::new();

    let decoded_base64_to_bytes=decode_base64(&base64input);    
    
    for byte3 in &decoded_base64_to_bytes {
        for i in (1..4).rev() {
            let mut curr_byte3_numerator = byte3.clone();
            if i < 3 {
                curr_byte3_numerator %= 256u32.pow(i);
            }

            let byte = curr_byte3_numerator / 256u32.pow(i - 1);
            bytes.push(byte as u8);
        }
    }
    
    bytes
}

pub fn format_base64_string(base64input: &str)->Result<&str, &str>
{
    let base64input = base64input.clone().trim_right_matches('=');
    
    let mut contains=false;
    for i in 0..base64input.len()
    {
        contains=BASE64CHARS.contains(&&base64input[i..i+1]);
        if contains==false
        {
            break;
        }
    }
    
    match contains
    {
        true=>Ok(base64input),
        false=>Err("Error.. An invalid base 64 string was input!")
    }
}

//Tests the get hamming distance function
#[test]
fn test_get_hamming_distance() {
  let bin_s1=b"this is a test";
  let bin_s2=b"wokka wokka!!!";
  
  assert_eq!(37, get_hamming_distance(bin_s1, bin_s2));
}

//Tests the decode base64 function.
#[test]
fn test_decode_base64()
{
    let base64string0="AAAA";
    let base64string1="AAAB";
    let base64string2="////";
    let base64string3="Sd+5";
    let base64string4=format_base64_string("AAAB=");
    let base64string5=format_base64_string("AAA=B=");
    
    assert_eq!(vec![0], decode_base64(base64string0));
    assert_eq!(vec![1], decode_base64(base64string1));
    assert_eq!(vec![16777215], decode_base64(base64string2));
    assert_eq!(vec![4841401], decode_base64(base64string3));
    assert!(base64string4.is_ok() && !base64string4.is_err());
    assert_eq!(vec![1], decode_base64(base64string4.unwrap()));
    assert!(base64string5.is_err() && !base64string5.is_ok());
}

//Tests the decoded base64 to bytes function.
#[test]
fn test_decoded_base64_to_bytes()
{
    let base64string0="////";
    let base64string1="AAAAAAAA";
    let base64string2="AAABAAAB";
    let base64string3="Sd+5";
    let base64string4=format_base64_string("AAAB=");
    let base64string5=format_base64_string("AAA&B=");
    
    assert_eq!(vec![255, 255, 255], decoded_base64_to_bytes(base64string0));
    assert_eq!(vec![0, 0, 0, 0, 0, 0], decoded_base64_to_bytes(base64string1));
    assert_eq!(vec![0, 0, 1, 0, 0, 1], decoded_base64_to_bytes(base64string2));
    assert_eq!(vec![73, 223, 185], decoded_base64_to_bytes(base64string3));
    assert!(base64string4.is_ok() && !base64string4.is_err());
    assert_eq!(vec![0, 0, 1], decoded_base64_to_bytes(base64string4.unwrap()));
    assert!(base64string5.is_err() && !base64string5.is_ok());
}






