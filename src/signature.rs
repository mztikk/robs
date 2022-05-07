use std::{error, fmt, usize};

use stringr::Stringr;

pub struct Signature {
    pub first_wildcard: Option<usize>,
    pub first_byte: Option<usize>,
    pub length: usize,
    pub pattern: Vec<u8>,
    pub mask: Vec<char>,
    pub sig: String,
    pub offset: usize,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &Signature::format(&self.sig)?)
    }
}

#[derive(Debug, Clone)]
pub struct SignatureLengthError;
impl error::Error for SignatureLengthError {}

impl fmt::Display for SignatureLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "signature length(excluding whitespace) must be divisible by 2, make sure to prepend bytes with 0 if necessary and make wildcards full ?? instead of single ?")
    }
}

impl From<SignatureLengthError> for fmt::Error {
    fn from(_: SignatureLengthError) -> Self {
        fmt::Error
    }
}

impl Signature {
    pub fn get_pattern_and_mask_from_signature(
        signature: &str,
    ) -> Result<(Vec<u8>, Vec<char>), Box<dyn error::Error>> {
        let signature = signature.remove_whitespace();
        if signature.len() % 2 != 0 {
            Err(SignatureLengthError.into())
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
                        Err(e) => return Err(e.into()),
                    }
                    mask.push('x');
                }
            }
            Ok((bytes, mask))
        }
    }

    pub fn new(signature: &str, offset: usize) -> Result<Signature, Box<dyn error::Error>> {
        let (pattern, mask) = Signature::get_pattern_and_mask_from_signature(signature)?;

        return Ok(Signature {
            first_wildcard: mask.iter().position(|&c| c == '?'),
            first_byte: mask.iter().position(|&c| c == 'x'),
            length: pattern.len(),
            pattern,
            mask,
            sig: Signature::format(signature)?,
            offset,
        });
    }

    pub fn format(signature: &str) -> Result<String, SignatureLengthError> {
        let signature = signature.remove_whitespace();
        if signature.len() % 2 != 0 {
            return Err(SignatureLengthError);
        }

        Ok(signature.splitn_separator(2, " ").to_ascii_uppercase())
    }
}
