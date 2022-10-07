use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len() - 1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
fn run() -> Result<(), Box<dyn Error>> {
    let filename = "solidity/contract.bin";
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    println!("{}\n", buffer);
    let bytecode = decode(&buffer)?;
    for b in bytecode {
        print!("0x{:x}", b);
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
