
extern crate svd_parser;

use std::*;
use std::cmp::*;

use types::traits::*;
use types::register_info::*;


// FIXME: assuming arrays with the same base element are equal
#[derive(Ord, Eq, PartialOrd, PartialEq, Clone, Debug)]
pub struct Peripheral(Vec<RegisterInfo>);

impl Peripheral {
    pub fn new(
        registers: Vec<svd_parser::Register>,
        defaults: &svd_parser::Defaults,
    ) -> Result<Peripheral, String> {
        registers
            .into_iter()
            .map(|register| {
                RegisterInfo::new(
                    match register {
                        svd_parser::Register::Single(info) => info,
                        svd_parser::Register::Array(info, _) => info,
                    },
                    defaults,
                )
            })
            .collect::<Result<Vec<RegisterInfo>, String>>()
            .map(Peripheral)
    }

    pub fn find_register(&self, name: &str) -> Option<RegisterInfo> {
        for ri in &(self.0) {
            if ri.name == name {
                return Some(ri.clone());
            }
        }
        return None;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Subset for Peripheral {
    fn is_subset(&self, other: &Self) -> bool {
        for reg in &(self.0) {
            if !match other.find_register(&reg.name) {
                Some(r) => *reg == r,
                None => false,
            } { return false; }
        }
        true
    }
}

impl Superset for Peripheral {
    fn is_superset(&self, other: &Self) -> bool {
        for reg in &(other.0) {
            if !match self.find_register(&reg.name) {
                Some(r) => *reg == r,
                None => false,
            } { return false; }
        }
        true
    }
}

impl Common for Peripheral {
    fn extract_common(&self, other: &Self) -> Self {
        let mut periph = self.clone();

        periph.0 = self.0.iter().filter_map(|r| other.find_register(&r.name) ).collect();

        periph
    }
}

impl Diff for Peripheral {
    fn diff(&self, other: &Self) -> Self {
        let mut periph = self.clone();

        periph.0 = other.0.iter().filter_map(|r| {
            match self.find_register(&r.name) {
                Some(r) => None,
                None => Some(r.clone()),
            }
        }).collect();

        periph
    }
}