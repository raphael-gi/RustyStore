use crate::base64_encode;

#[test]
fn test_encoding() {
    assert_eq!(base64_encode(r#"{""#.to_string()), "eyI")
}
