pub mod scanner;
pub mod signature;

#[cfg(test)]
mod tests {
    #[test]
    fn sig() {
        let strsig = "AE FF ?? DE";
        let mask = vec!['x', 'x', '?', 'x'];
        let sig = crate::signature::Signature::new(strsig, 0).unwrap();
        assert_eq!(sig.sig, strsig);
        assert_eq!(sig.mask, mask);
    }

    #[test]
    fn scan_last() {
        let bytes = [0x0, 0xFF];
        let sig = crate::signature::Signature::new(&String::from("FF"), 0).unwrap();
        let find = crate::scanner::find_signature(&bytes, &sig).unwrap();
        assert_eq!(find, 1);
    }

    use crate::scanner::AobScanner;

    #[test]
    fn scan_first() {
        let strsig = "FF";
        let bytes = [0xFF_u8, 0x0];
        let sig = crate::signature::Signature::new(strsig, 0).unwrap();
        let find = bytes.find_signature(&sig).unwrap();
        assert_eq!(find, 0);
    }
}
