use ruth_cli::config::Config;
use std::path::PathBuf;

#[test]
fn test_default_store_path() {
    std::env::remove_var("RUTH_STORE");
    std::env::remove_var("RUTH_GPG_ID");
    let config = Config::resolve(None, None, None);
    let expected = dirs::config_dir().unwrap().join("ruth-cli").join("store.toml.gpg");
    assert_eq!(config.store_path, expected);
}

#[test]
fn test_cli_flag_overrides_store_path() {
    let custom = PathBuf::from("/tmp/my-store.toml.gpg");
    let config = Config::resolve(Some(custom.clone()), None, None);
    assert_eq!(config.store_path, custom);
}

#[test]
fn test_gpg_key_from_param() {
    let config = Config::resolve(None, Some("me@example.com".to_string()), None);
    assert_eq!(config.gpg_key_id, Some("me@example.com".to_string()));
}
