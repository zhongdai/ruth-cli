use ruth_cli::store::{Entry, Store};
use ruth_cli::totp::Algorithm;

#[test]
fn test_add_entry() {
    let mut store = Store::default();
    let entry = Entry {
        label: "github-work".to_string(),
        domain: "github.com".to_string(),
        account: "user@work.com".to_string(),
        secret: "JBSWY3DPEHPK3PXP".to_string(),
        algorithm: Algorithm::SHA1,
        digits: 6,
        period: 30,
    };
    store.add(entry).unwrap();
    assert_eq!(store.entries().len(), 1);
    assert_eq!(store.entries()[0].label, "github-work");
}

#[test]
fn test_add_duplicate_label_fails() {
    let mut store = Store::default();
    let entry = Entry {
        label: "test".to_string(),
        domain: "example.com".to_string(),
        account: "user@example.com".to_string(),
        secret: "JBSWY3DPEHPK3PXP".to_string(),
        algorithm: Algorithm::SHA1,
        digits: 6,
        period: 30,
    };
    store.add(entry.clone()).unwrap();
    let result = store.add(entry);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("test"));
}

#[test]
fn test_remove_entry() {
    let mut store = Store::default();
    let entry = Entry {
        label: "test".to_string(),
        domain: "example.com".to_string(),
        account: "user@example.com".to_string(),
        secret: "JBSWY3DPEHPK3PXP".to_string(),
        algorithm: Algorithm::SHA1,
        digits: 6,
        period: 30,
    };
    store.add(entry).unwrap();
    store.remove("test").unwrap();
    assert!(store.entries().is_empty());
}

#[test]
fn test_remove_nonexistent_fails() {
    let mut store = Store::default();
    let result = store.remove("nope");
    assert!(result.is_err());
}

#[test]
fn test_find_entry() {
    let mut store = Store::default();
    let entry = Entry {
        label: "test".to_string(),
        domain: "example.com".to_string(),
        account: "user@example.com".to_string(),
        secret: "JBSWY3DPEHPK3PXP".to_string(),
        algorithm: Algorithm::SHA1,
        digits: 6,
        period: 30,
    };
    store.add(entry).unwrap();
    assert!(store.find("test").is_some());
    assert!(store.find("nope").is_none());
}

#[test]
fn test_list_labels() {
    let mut store = Store::default();
    for label in &["aaa", "bbb", "ccc"] {
        store
            .add(Entry {
                label: label.to_string(),
                domain: "example.com".to_string(),
                account: "u@e.com".to_string(),
                secret: "JBSWY3DPEHPK3PXP".to_string(),
                algorithm: Algorithm::SHA1,
                digits: 6,
                period: 30,
            })
            .unwrap();
    }
    let labels = store.labels();
    assert_eq!(labels, vec!["aaa", "bbb", "ccc"]);
}

#[test]
fn test_serialize_deserialize_roundtrip() {
    let mut store = Store::default();
    store
        .add(Entry {
            label: "test".to_string(),
            domain: "example.com".to_string(),
            account: "u@e.com".to_string(),
            secret: "JBSWY3DPEHPK3PXP".to_string(),
            algorithm: Algorithm::SHA256,
            digits: 8,
            period: 60,
        })
        .unwrap();

    let toml_str = store.to_toml().unwrap();
    let restored = Store::from_toml(&toml_str).unwrap();
    assert_eq!(restored.entries().len(), 1);
    assert_eq!(restored.entries()[0].algorithm, Algorithm::SHA256);
    assert_eq!(restored.entries()[0].digits, 8);
    assert_eq!(restored.entries()[0].period, 60);
}

#[test]
fn test_add_validates_secret() {
    let mut store = Store::default();
    let entry = Entry {
        label: "bad".to_string(),
        domain: "example.com".to_string(),
        account: "u@e.com".to_string(),
        secret: "NOT-VALID!!!".to_string(),
        algorithm: Algorithm::SHA1,
        digits: 6,
        period: 30,
    };
    let result = store.add(entry);
    assert!(result.is_err());
}
