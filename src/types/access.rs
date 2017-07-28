extern crate svd_parser;

use std::*;
use std::cmp::*;


#[derive(Ord, Eq, PartialOrd, PartialEq, Clone, Debug)]
pub struct Access(pub u32);

impl From<svd_parser::Access> for Access {
    fn from(access: svd_parser::Access) -> Access {
        Access(access as u32)
    }
}
