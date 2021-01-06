extern crate nom;
extern crate nom_derive;

mod header;
mod types;

use std::env;
use std::fs;

use header::ElfHeader;

fn read_header(raw: Vec<u8>) {
    let slice = raw.as_slice();
    let parsed = ElfHeader::parse(&slice);

    match parsed {
        Ok((x, y)) => println!("{:?}", y),
        Err(_) => panic!("Unable to parse ELF header"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];
    println!("{}", filename);

    match fs::read(filename) {
        Ok(x) => read_header(x),
        Err(_) => panic!("Could not read file '{}'", filename),
    }
}
