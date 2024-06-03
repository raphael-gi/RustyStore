use crate::{base64_decode, base64_encode};

#[test]
fn test_encoding() {
    assert_eq!(base64_encode("abc".to_string()), "YWJj")
}

#[test]
fn test_decoding() {
    assert_eq!(base64_decode("YWJj".to_string()), "abc")
}

#[test]
fn test_encoding_and_decoding() {
    let input: &str = "ALSDJFOIW&ç%&/**%jaölndasfoiaefjaöfjalfjfjskdfjdsfjdkfj";
    let encoded = base64_encode(input.to_string());
    assert_eq!(input.to_string(), base64_decode(encoded))
}
