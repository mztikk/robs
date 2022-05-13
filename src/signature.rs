use std::{fmt, usize};
use stringr::Stringr;
use thiserror::Error;

#[derive(Debug)]
pub struct Signature {
    pub first_wildcard: Option<usize>,
    pub first_byte: Option<usize>,
    pub matching_indices: Vec<usize>,
    pub length: usize,
    pub pattern: Vec<u8>,
    pub mask: Vec<char>,
    pub sig: String,
    pub offset: usize,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match Signature::format(&self.sig) {
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SignatureParseError {
    #[error(
        "Signature length(excluding whitespace) must be divisible by 2, but was of length {0}"
    )]
    InvalidLength(usize),
    #[error("Signature must contain only hexadecimal characters, but was {0}")]
    InvalidString(String),
}

impl Signature {
    pub fn get_pattern_and_mask_from_signature(
        signature: &str,
    ) -> Result<(Vec<u8>, Vec<char>), SignatureParseError> {
        let signature = signature.remove_whitespace();
        if signature.len() % 2 != 0 {
            Err(SignatureParseError::InvalidLength(signature.len()))
        } else {
            let split = signature.splitn(2);
            let mut bytes = Vec::with_capacity(split.len());
            let mut mask = Vec::with_capacity(split.len());
            for c in split {
                if c.contains('?') {
                    bytes.push(0);
                    mask.push('?');
                } else {
                    match u8::from_str_radix(&c, 16) {
                        Ok(v) => {
                            bytes.push(v);
                        }
                        Err(_e) => return Err(SignatureParseError::InvalidString(c)),
                    }
                    mask.push('x');
                }
            }
            Ok((bytes, mask))
        }
    }

    pub fn new(signature: &str, offset: usize) -> Result<Signature, SignatureParseError> {
        let (pattern, mask) = Signature::get_pattern_and_mask_from_signature(signature)?;
        let indices = mask
            .iter()
            .enumerate()
            .filter_map(|(i, &m)| if m == 'x' { Some(i) } else { None })
            .collect();

        return Ok(Signature {
            first_wildcard: mask.iter().position(|&c| c == '?'),
            first_byte: mask.iter().position(|&c| c == 'x'),
            matching_indices: indices,
            length: pattern.len(),
            pattern,
            mask,
            sig: Signature::format(signature)?,
            offset,
        });
    }

    pub fn format(signature: &str) -> Result<String, SignatureParseError> {
        let signature = signature.remove_whitespace();
        if signature.len() % 2 != 0 {
            return Err(SignatureParseError::InvalidLength(signature.len()));
        }

        Ok(signature.splitn_separator(2, " ").to_ascii_uppercase())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_invalid_length() {
        let signature = "123";
        let signature = super::Signature::new(signature, 0);
        assert!(signature.is_err());
        assert_eq!(
            signature.unwrap_err(),
            super::SignatureParseError::InvalidLength(3)
        );
    }

    #[test]
    fn test_invalid_string() {
        let signature = "zz";
        let signature = super::Signature::new(signature, 0);
        assert!(signature.is_err());
        assert_eq!(
            signature.unwrap_err(),
            super::SignatureParseError::InvalidString("zz".to_string())
        );
    }
}
