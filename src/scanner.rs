use std::usize;

use crate::signature::Signature;

pub fn find_signature(search_region: &Vec<u8>, signature: &Signature) -> Option<usize> {
    let first_index = signature.first_byte.unwrap();
    let first_item = signature.pattern[first_index];

    let mut i: usize = 0;
    let upper_bound = search_region.len() - signature.pattern.len();
    while i < upper_bound {
        let find = match search_region[i..].iter().position(|&x| x == first_item) {
            Some(v) => v,
            None => return None,
        };

        i += find;

        let (check, delta) = check_mask(&search_region[i..], &signature);
        if check {
            return Some(i + signature.offset);
        }

        i += delta;
    }

    return None;
}

fn check_mask(search_region: &[u8], signature: &Signature) -> (bool, usize) {
    let len = signature.pattern.len();
    for i in 0..len {
        if signature.mask[i] != '?' && signature.pattern[i] != search_region[i] {
            return (false, i);
        }
    }

    return (true, len);
}
