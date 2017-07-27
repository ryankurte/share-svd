extern crate svd_parser;

use std::*;
use std::cmp::*;

#[derive(Ord, Eq, PartialOrd, PartialEq, Clone)]
pub struct BitRange {
    pub offset: u32,
    pub width: u32,
}

impl From<svd_parser::BitRange> for BitRange {
    fn from(other: svd_parser::BitRange) -> BitRange {
        BitRange {
            offset: other.offset,
            width: other.width,
        }
    }
}