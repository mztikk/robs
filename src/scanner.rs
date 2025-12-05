use crate::signature::Signature;
use rayon::prelude::*;
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
        .par_iter()
        .positions(|&item| item == first_item)
        .filter(|&index| index + mask_len <= search_region.len())
        .find_any(|&index| {
            check_mask(
                &search_region[(index - first_index)..(index + mask_len)],
                signature,
            )
        })
        .map(|index| index + signature.offset)
}

fn check_mask(search_region: &[u8], signature: &Signature) -> bool {
    signature
        .matching_index_ranges
        .iter()
        .all(|range| &search_region[range.clone()] == &signature.pattern[range.clone()])
}

#[cfg(test)]
mod tests {
    use crate::{scanner::check_mask, signature::Signature};

    #[test]
    fn test_check_mask() {
        let search_region = &[0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
        let signature = Signature::new("0B ?? 0D", 0);
        assert!(signature.is_ok());
        let signature = signature.unwrap();
        let check = check_mask(search_region, &signature);
        assert!(check);
    }
}
