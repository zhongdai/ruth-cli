use ruth_cli::qr::parse_otpauth_uri;

#[test]
fn test_parse_full_uri() {
    let uri = "otpauth://totp/GitHub:user@example.com?secret=JBSWY3DPEHPK3PXP&issuer=GitHub&algorithm=SHA256&digits=8&period=60";
    let parsed = parse_otpauth_uri(uri).unwrap();
    assert_eq!(parsed.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(parsed.issuer, Some("GitHub".to_string()));
    assert_eq!(parsed.account, Some("user@example.com".to_string()));
    assert_eq!(parsed.algorithm, "SHA256");
    assert_eq!(parsed.digits, 8);
    assert_eq!(parsed.period, 60);
}

#[test]
fn test_parse_minimal_uri() {
    let uri = "otpauth://totp/MyService?secret=JBSWY3DPEHPK3PXP";
    let parsed = parse_otpauth_uri(uri).unwrap();
    assert_eq!(parsed.secret, "JBSWY3DPEHPK3PXP");
    assert_eq!(parsed.algorithm, "SHA1");
    assert_eq!(parsed.digits, 6);
    assert_eq!(parsed.period, 30);
}

#[test]
fn test_parse_missing_secret_fails() {
    let uri = "otpauth://totp/MyService?issuer=Foo";
    assert!(parse_otpauth_uri(uri).is_err());
}

#[test]
fn test_parse_not_totp_fails() {
    let uri = "otpauth://hotp/MyService?secret=JBSWY3DPEHPK3PXP";
    assert!(parse_otpauth_uri(uri).is_err());
}

#[test]
fn test_auto_label_issuer_account() {
    let uri = "otpauth://totp/GitHub:user@example.com?secret=JBSWY3DPEHPK3PXP&issuer=GitHub";
    let parsed = parse_otpauth_uri(uri).unwrap();
    assert_eq!(parsed.auto_label(), "GitHub-user@example.com");
}

#[test]
fn test_auto_label_issuer_only() {
    let uri = "otpauth://totp/GitHub?secret=JBSWY3DPEHPK3PXP&issuer=GitHub";
    let parsed = parse_otpauth_uri(uri).unwrap();
    assert_eq!(parsed.auto_label(), "GitHub");
}

#[test]
fn test_auto_label_account_only() {
    let uri = "otpauth://totp/user@example.com?secret=JBSWY3DPEHPK3PXP";
    let parsed = parse_otpauth_uri(uri).unwrap();
    assert_eq!(parsed.auto_label(), "user@example.com");
}
