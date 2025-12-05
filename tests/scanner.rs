use robs::{scanner::find_signature, signature::Signature};

#[test]
fn test_find_signature() {
    let search_region = &[
        0x00, 0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
        0x0E, 0x0F,
    ];
    let signature = Signature::new("0B ?? 0D", 0);
    assert!(signature.is_ok());
    let signature = signature.unwrap();
    let find = find_signature(search_region, &signature);
    assert!(find.is_some());
    let find = find.unwrap();
    assert_eq!(find, 12);
}

#[test]
fn test_signature_not_found() {
    let search_region = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let signature = Signature::new("FF FF", 0);
    assert!(signature.is_ok());
    let signature = signature.unwrap();
    let find = find_signature(search_region, &signature);
    assert!(find.is_none());
}

#[test]
fn test_signature_exact() {
    let search_region = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let signature = Signature::new("00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F", 0);
    assert!(signature.is_ok());
    let signature = signature.unwrap();
    let find = find_signature(search_region, &signature);
    assert!(find.is_some());
    let find = find.unwrap();
    assert_eq!(find, 0);
}

#[test]
fn test_signature_exact_except_last() {
    let search_region = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F,
    ];
    let signature = Signature::new("00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E FF", 0);
    assert!(signature.is_ok());
    let signature = signature.unwrap();
    let find = find_signature(search_region, &signature);
    assert!(find.is_none());
}

#[test]
fn test_signature_start_with_wildcard() {
    let search_region = &[
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0xFF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01,
    ];
    let signature = Signature::new("?? 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F", 0);
    assert!(signature.is_ok());
    let signature = signature.unwrap();
    let find = find_signature(search_region, &signature);
    assert!(find.is_some());
    let find = find.unwrap();
    assert_eq!(find, 1);
}
