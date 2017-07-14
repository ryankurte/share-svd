extern crate svd_parser;
extern crate getopts;
extern crate threadpool;

use std::{vec,fs};
use std::path::Path;
use std::collections::BTreeMap;
use std::env::args;
use std::io;

use getopts::Options;
use std::env;

mod loader;
use loader::*;

mod types;
use types::*;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Setup command line options
    let mut opts = Options::new();
    opts.optopt("i", "indir", "SVD input directory", "INDIR");
    opts.optopt("o", "outdir", "output directory", "OUTDIR");
    opts.optopt("m", "mode", "set operating mode", "MODE");
    opts.optflag("v", "verbose", "Verbose output mode");
    opts.optflag("h", "help", "print this help menu");

    // Parse and handle command line options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let verbose = matches.opt_present("v");
    let dir_in = match matches.opt_str("i") {
        Some(o) => o.clone(),
        None => String::from("svd")
    };
    let dir_out = match matches.opt_str("o") {
        Some(o) => o.clone(),
        None => String::from("src")
    };
     
    // Load devices into memory by family (directory based)
    let (families, device_count) = match load_families(&dir_in) {
        Ok(f) => { f }
        Err(f) => { panic!(f.to_string()) }
    };

    println!("Loaded {:?} devices", device_count);

    extract_peripherals(families);

}

type PeripheralList = Vec<(String, Peripheral)>;

// Load peripherals for a family from a DeviceList
fn get_family_peripherals(devices: DeviceList) -> PeripheralList {
    let mut family_peripherals: PeripheralList = Vec::new();

    for d in devices {
        let device_name = d.0;
        let device_inst = d.1;
        let defaults = device_inst.defaults.clone();
        println!("Fetching peripherals for: {:?}", device_name);

        let peripherals: PeripheralList = device_inst.peripherals.into_iter().flat_map(move |p| {
            let path_name = format!("{}:{}", &device_name, p.name);
            p.registers.map(move |registers| {
                (path_name, Peripheral::new(registers, &defaults).unwrap())
            })
        }).collect();

        family_peripherals.extend(peripherals.into_iter());
    }

    return family_peripherals;
}

// Deduplicate a list of peripherals
fn deduplicate_peripherals(peripherals: PeripheralList) {
    let mut set: BTreeMap<Peripheral, usize> = BTreeMap::new();

    for (n, p) in peripherals {
        match set.get_mut(&p) {
            Some(mut p) => *p += 1,
            None => set.insert(p, 0),
        }
    }
    

}

fn extract_peripherals(families: FamilyList) {

    for (name, devices) in families {
        println!("Extracting peripherals for {}", name);
        let family_peripherals = get_family_peripherals(devices);


    }

}