use super::JWT;

#[test]
fn test_build_json() {
    let jwt = JWT::new("heheheha".to_string())
        .add_header("alg", "HS256")
        .add_header("typ", "JWT")
        .add_payload("sub", "1234567890")
        .add_payload("name", "John Doe")
        .add_payload("iat", "1516239022")
        .build();

    assert_eq!(jwt, r#"{"alg":"HS256","typ":"JWT"}"#)
}

