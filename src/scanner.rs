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

        if check_mask(i, search_region, &signature) {
            return Some(i + signature.offset);
        }

        i += 1;
    }

    return None;
}

fn check_mask(index: usize, search_region: &Vec<u8>, signature: &Signature) -> bool {
    for i in 0..signature.pattern.len() {
        if signature.mask[i] != '?' && signature.pattern[i] != search_region[index + i] {
            return false;
        }
    }

    return true;
}
