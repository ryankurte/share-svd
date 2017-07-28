
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

pub struct DeviceInfo {
    pub name: String,
    pub device: svd_parser::Device
}

impl DeviceInfo {
    pub fn new(name: String, device: svd_parser::Device) -> DeviceInfo {
        DeviceInfo{name, device}
    }
}


pub type PeripheralList = Vec<PeripheralInfo>;

pub type DeviceList = Vec<(String, svd_parser::Device)>;

pub type FamilyList = Vec<(String, DeviceList)>;


#[derive(Ord, Eq, PartialOrd, PartialEq, Clone, Debug)]
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

#[derive(Debug)]
pub struct PeripheralNode {
    pub info: PeripheralInfo,
    pub children: Vec<PeripheralNode>
}

impl PeripheralNode {
    pub fn reduce() {

    }
}

#[derive(Debug)]
pub struct PeripheralMap {
    //pub base: PeripheralNode
}

impl From<PeripheralList> for PeripheralMap {
    fn from(peripherals: PeripheralList) -> PeripheralMap {
        if peripherals.len() == 1 {
            return PeripheralMap{}
        }

        let nodes: Vec<PeripheralNode> = peripherals.clone().iter().map(|p| { 
                PeripheralNode{info: p.clone(), children: Vec::new()}
        }).collect();
        let mut dependents: Vec<usize> = Vec::new();

        // Extract commonalities from peripheral list
        let mut common = nodes[0].info.peripheral.clone();
        for (i, n) in (&nodes).iter().enumerate() {
            if i == 0 {
                continue;
            }
            common = common.extract_common(&n.info.peripheral);
        }
        print!("\t\t\t\tCommon: {} regs, diffs: [", common.len());

        // Find diffs against commonality
        for (i, n) in (&nodes).iter().enumerate() {
            let diff = common.diff(&n.info.peripheral);
            print!("({}: {} regs) ", n.info.device, diff.len());
        }
        println!("]");

       return PeripheralMap{}
    }
}


impl PeripheralMap {
    
}