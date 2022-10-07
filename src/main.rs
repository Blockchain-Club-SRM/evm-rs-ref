use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn run() -> Result<(), Box<dyn Error>> {
    let filename = "solidity/contract.bin";
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn main() {
    run().unwrap();
}
