
extern crate flate2;

use std::env::args;
use std::fs::File;
use std::time::*;
use std::io::*;
use flate2::Compression;
use flate2::write::GzEncoder;

#[allow(dead_code, unused_variables)]
pub fn proceed() {
    if args().len() != 3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }

    let mut source = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let target = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(target, Compression::default());

    let start = Instant::now();
    copy(&mut source, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();
    println!("Elapsed: {:?}", start.elapsed());
    print!("Compressed: {} -> {} ({}%)\n",
           source.get_ref().metadata().unwrap().len(),
           output.metadata().unwrap().len(),
           output.metadata().unwrap().len() * 100 / source.get_ref().metadata().unwrap().len());
    println!(":D")
}