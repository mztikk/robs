use std::{ops::Index, usize};

use crate::signature::Signature;

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
    let first_index = signature.first_byte.unwrap();
    let first_item = signature.pattern[first_index];

    let mut i: usize = 0;
    while i < search_region.len() - signature.pattern.len() {
        let find = search_region[i..].iter().position(|&x| x == first_item)?;

        i += find;

        let (check, delta) = check_mask(&search_region[i..], &signature);
        if check {
            return Some(i + signature.offset);
        }

        i += delta;
    }

    None
}

fn check_mask<T: ?Sized>(search_region: &T, signature: &Signature) -> (bool, usize)
where
    T: Index<usize, Output = u8>,
{
    let len = signature.pattern.len();
    for i in 0..len {
        if signature.mask[i] != '?' && signature.pattern[i] != search_region[i] {
            return (false, i);
        }
    }

    (true, len)
}
