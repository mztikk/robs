use robs::signature::{Signature, SignatureParseError};

#[test]
fn test_invalid_length() {
    let signature = "123";
    let signature = Signature::new(signature, 0);
    assert!(signature.is_err());
    assert_eq!(
        signature.unwrap_err(),
        SignatureParseError::InvalidLength(3)
    );
}

#[test]
fn test_invalid_string() {
    let signature = "zz";
    let signature = Signature::new(signature, 0);
    assert!(signature.is_err());
    assert_eq!(
        signature.unwrap_err(),
        SignatureParseError::InvalidString("zz".to_string())
    );
}

#[test]
fn test_valid_signature() {
    let signature = "12 34 56 78";
    let signature = Signature::new(signature, 0);
    assert!(signature.is_ok());
    assert_eq!(
        signature.unwrap(),
        Signature {
            first_wildcard: None,
            first_byte: Some(0),
            matching_indices: vec![0, 1, 2, 3],
            matching_index_ranges: vec![0..4],
            length: 4,
            pattern: vec![0x12, 0x34, 0x56, 0x78],
            mask: vec!['x', 'x', 'x', 'x'],
            sig: "12 34 56 78".to_string(),
            offset: 0,
        }
    );
}

#[test]
fn test_valid_signature_with_wildcards() {
    let signature = "12 ?? 56 78";
    let signature = Signature::new(signature, 0);
    assert!(signature.is_ok());
    assert_eq!(
        signature.unwrap(),
        Signature {
            first_wildcard: Some(1),
            first_byte: Some(0),
            matching_indices: vec![0, 2, 3],
            matching_index_ranges: vec![0..1, 2..4],
            length: 4,
            pattern: vec![0x12, 0x00, 0x56, 0x78],
            mask: vec!['x', '?', 'x', 'x'],
            sig: "12 ?? 56 78".to_string(),
            offset: 0,
        }
    );
}

#[test]
fn test_formatting() {
    let signatures = vec![
        ("abcd", "AB CD"),
        ("a b c D", "AB CD"),
        ("a b c d 12 34 56 78", "AB CD 12 34 56 78"),
    ];
    for (sig, formatted) in signatures {
        let signature = Signature::format(sig);
        assert!(signature.is_ok());
        let signature = signature.unwrap();
        assert_eq!(signature, formatted);
    }
}
