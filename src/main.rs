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
    let families = match load_families(&dir_in) {
        Ok(f) => { f }
        Err(f) => { panic!(f.to_string()) }
    };

    println!("Loaded {:?} devices", families.1);

}
