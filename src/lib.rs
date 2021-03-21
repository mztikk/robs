pub mod scanner;
pub mod signature;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn sig() {
        let strsig = "AEFF??DE";
        let mask = vec!['x', 'x', '?', 'x'];
        let sig = crate::signature::Signature::new(strsig.to_string(), 0).unwrap();
        assert_eq!(sig.sig, strsig);
        assert_eq!(sig.mask, mask);
    }
}
