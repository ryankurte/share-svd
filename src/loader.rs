
extern crate svd_parser;
extern crate threadpool;
extern crate num_cpus;

use std::{vec,fs, time};
use std::path::Path;
use std::collections::BTreeMap;
use std::env::args;
use std::io;
use std::io::Write;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use types::*;

pub fn load_families(path: &str) -> io::Result<(FamilyList, usize)> {
    let families: Vec<String> = fs::read_dir(&path).unwrap()
        .map(|res| res.unwrap())
        .filter(|f| f.metadata().unwrap().is_dir())
        .map(|f| f.file_name().into_string().unwrap())
        .collect();

    println!("Discovered families: {:?}", families);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    let jobs: usize = families.len();
    let now = time::Instant::now();
   
    let mut family_list: FamilyList = Vec::new();
    let mut device_count = 0;

    println!("Loading");

    for f in families {
        let tx = tx.clone();
        let name = f.clone();
        let file = format!("{}/{}", path, f);

        pool.execute(move|| {
            let devices = load_devices(&file).unwrap();
            tx.send((name, devices)).unwrap();
        });
    }

    for d in rx.iter().take(jobs) {
        device_count += d.1.len();
        family_list.push(d);
    }

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1e9);

    println!("\nLoaded {0} devices from {1} families in {2:.2} seconds", device_count, family_list.len(), sec);

    Ok((family_list, device_count))
}

pub fn load_devices(path: &str) -> io::Result<DeviceList> {
    let svd_list = fs::read_dir(path).unwrap()
            .map(|res| res.unwrap())
            .filter(|f| !f.metadata().unwrap().is_dir())
            .map(|f| f.file_name().into_string().unwrap())
            .map(|f| f.replace(".svd", ""));
    
    let mut devices: DeviceList = Vec::new();

    for s in svd_list {
        let p = format!("{}/{}.svd", path, &s);
        let d = load_device(&p)?;
        devices.push((s, d));
        print!(".");
        io::stdout().flush();
    }
    Ok(devices)
}

pub fn load_device(path: &str) -> io::Result<svd_parser::Device> {
    use std::io::Read;
    let mut xml = String::new();
    let mut f = fs::File::open(&path)?;
    f.read_to_string(&mut xml)?;
    Ok(svd_parser::parse(xml.as_ref()))
}
