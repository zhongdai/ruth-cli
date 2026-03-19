use ruth_cli::totp::{generate, validate_secret, Algorithm};

const SHA1_SECRET: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
const SHA256_SECRET: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA====";
const SHA512_SECRET: &str = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA=";

#[test]
fn test_rfc6238_sha1_59() {
    let code = generate(SHA1_SECRET, 59, 30, 8, Algorithm::SHA1).unwrap();
    assert_eq!(code, "94287082");
}

#[test]
fn test_rfc6238_sha256_59() {
    let code = generate(SHA256_SECRET, 59, 30, 8, Algorithm::SHA256).unwrap();
    assert_eq!(code, "46119246");
}

#[test]
fn test_rfc6238_sha512_59() {
    let code = generate(SHA512_SECRET, 59, 30, 8, Algorithm::SHA512).unwrap();
    assert_eq!(code, "90693936");
}

#[test]
fn test_rfc6238_sha1_1111111109() {
    let code = generate(SHA1_SECRET, 1111111109, 30, 8, Algorithm::SHA1).unwrap();
    assert_eq!(code, "07081804");
}

#[test]
fn test_rfc6238_sha256_1111111109() {
    let code = generate(SHA256_SECRET, 1111111109, 30, 8, Algorithm::SHA256).unwrap();
    assert_eq!(code, "68084774");
}

#[test]
fn test_rfc6238_sha512_1111111109() {
    let code = generate(SHA512_SECRET, 1111111109, 30, 8, Algorithm::SHA512).unwrap();
    assert_eq!(code, "25091201");
}

#[test]
fn test_rfc6238_sha1_1234567890() {
    let code = generate(SHA1_SECRET, 1234567890, 30, 8, Algorithm::SHA1).unwrap();
    assert_eq!(code, "89005924");
}

#[test]
fn test_6_digit_output() {
    let code = generate(SHA1_SECRET, 59, 30, 6, Algorithm::SHA1).unwrap();
    assert_eq!(code.len(), 6);
}

#[test]
fn test_invalid_base32_secret() {
    let result = generate("NOT-VALID-BASE32!!!", 59, 30, 6, Algorithm::SHA1);
    assert!(result.is_err());
}

#[test]
fn test_validate_secret_valid() {
    assert!(validate_secret("JBSWY3DPEHPK3PXP").is_ok());
}

#[test]
fn test_validate_secret_invalid() {
    assert!(validate_secret("NOT-VALID!!!").is_err());
}
