
extern crate svd_parser;

use std::*;
use std::cmp::*;

mod traits;
pub use self::traits::*;
mod fields;
pub use self::fields::*;
mod bit_range;
pub use self::bit_range::*;
mod access;
pub use self::access::*;
mod register_info;
pub use self::register_info::*;
mod peripheral;
pub use self::peripheral::*;


#[derive(Ord, Eq, PartialOrd, PartialEq, Clone)]
pub struct PeripheralInfo {
    pub name: String,
    pub device: String,
    pub count: usize,
    pub peripheral: Peripheral
}

impl PeripheralInfo {
    pub fn new(name: String, device: String, count: usize, peripheral: Peripheral) -> PeripheralInfo {
        PeripheralInfo{name, device, count, peripheral}
    }
}

pub type PeripheralList = Vec<PeripheralInfo>;

pub type DeviceList = Vec<(String, svd_parser::Device)>;

pub type FamilyList = Vec<(String, DeviceList)>;
