use crate::signature::Signature;
use itertools::multizip;
use std::usize;

pub trait AobScanner {
    fn find_signature(&self, signature: &Signature) -> Option<usize>;
}

impl AobScanner for &[u8] {
    fn find_signature(&self, signature: &Signature) -> Option<usize> {
        crate::scanner::find_signature(self, signature)
    }
}

impl AobScanner for [u8] {
    fn find_signature(&self, signature: &Signature) -> Option<usize> {
        crate::scanner::find_signature(self, signature)
    }
}

impl AobScanner for Vec<u8> {
    fn find_signature(&self, signature: &Signature) -> Option<usize> {
        crate::scanner::find_signature(self, signature)
    }
}

pub fn find_signature(search_region: &[u8], signature: &Signature) -> Option<usize> {
    let first_index = signature.first_byte?;
    let first_item = signature.pattern[first_index];
    let mask_len = signature.mask.len();

    search_region
        .iter()
        .position(|&item| item == first_item)
        .and_then(|index| {
            check_mask(&search_region[index..index + mask_len], signature)
                .then(|| index + signature.offset)
        })
}

fn check_mask(search_region: &[u8], signature: &Signature) -> bool {
    multizip((
        search_region.iter(),
        signature.mask.iter(),
        signature.pattern.iter(),
    ))
    .all(|(item, mask, pattern)| match mask {
        '?' => true,
        'x' => item == pattern,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use crate::{
        scanner::{check_mask, find_signature},
        signature::Signature,
    };

    #[test]
    fn test_check_mask() {
        let search_region = &[0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
        let signature = Signature::new("0B ?? 0D", 0);
        assert!(signature.is_ok());
        let signature = signature.unwrap();
        let check = check_mask(search_region, &signature);
        assert!(check);
    }

    #[test]
    fn test_find_signature() {
        let search_region = &[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let signature = Signature::new("0B ?? 0D", 0);
        assert!(signature.is_ok());
        let signature = signature.unwrap();
        let find = find_signature(search_region, &signature);
        assert!(find.is_some());
        let find = find.unwrap();
        assert_eq!(find, 11);
    }

    #[test]
    fn test_signature_not_found() {
        let search_region = &[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
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
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
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
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let signature = Signature::new("00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E FF", 0);
        assert!(signature.is_ok());
        let signature = signature.unwrap();
        let find = find_signature(search_region, &signature);
        assert!(find.is_none());
    }
}
