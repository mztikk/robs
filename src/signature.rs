use itertools::Itertools;
use std::{fmt, ops::Range, usize};
use stringr::{Stringr, splitn_byte};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    pub first_wildcard: Option<usize>,
    pub first_byte: Option<usize>,
    pub matching_indices: Vec<usize>,
    pub matching_index_ranges: Vec<Range<usize>>,
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
    #[error("Signature length(excluding whitespace) must be divisible by 2, but was of length {0}")]
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
            return Err(SignatureParseError::InvalidLength(signature.len()));
        }

        let split = splitn_byte(&signature, 2);
        let mut bytes = Vec::with_capacity(split.len());
        let mut mask = Vec::with_capacity(split.len());

        for c in split {
            if c.contains('?') {
                bytes.push(0);
                mask.push('?');
            } else {
                bytes.push(
                    u8::from_str_radix(c, 16)
                        .map_err(|_e| SignatureParseError::InvalidString(c.to_string()))?,
                );
                mask.push('x');
            }
        }

        Ok((bytes, mask))
    }

    pub fn new(signature: &str, offset: usize) -> Result<Signature, SignatureParseError> {
        let (pattern, mask) = Signature::get_pattern_and_mask_from_signature(signature)?;
        let indices = mask.iter().positions(|c| *c == 'x').collect();
        let matching_ranges = {
            let mut ranges = Vec::new();
            let mut start: Option<usize> = None;
            for (i, &c) in mask.iter().enumerate() {
                if c == 'x' {
                    if start.is_none() {
                        start = Some(i);
                    }
                } else {
                    if let Some(s) = start {
                        ranges.push(s..i);
                        start = None;
                    }
                }
            }
            if let Some(s) = start {
                ranges.push(s..mask.len());
            }
            ranges
        };

        return Ok(Signature {
            first_wildcard: mask.iter().position(|&c| c == '?'),
            first_byte: mask.iter().position(|&c| c == 'x'),
            matching_indices: indices,
            matching_index_ranges: matching_ranges,
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
