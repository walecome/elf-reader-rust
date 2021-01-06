extern crate nom;
extern crate nom_derive;

mod header;
mod types;

use std::env;
use std::fs;
use std::io;

use header::ElfHeader;

fn read_header(raw: Vec<u8>) {
    let slice = raw.as_slice();
    let parsed = ElfHeader::parse(&slice);
    // let parsed = MachineType::parse(b"\x00\x3e");

    // let parsed = MachineType::parse(b"\xFF\xFF");
    // dbg!(parsed);

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
