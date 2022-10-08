use super::opcode::Opcode;
use bigint::U256;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;

pub struct Vm {
    pub code: Vec<u8>,
    pub pc: usize,
    pub stack: Vec<U256>,
}

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..(s.len() - 1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

impl Vm {
    pub fn new_from_file(filename: &str) -> Result<Vm, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let bytecode = decode(&buffer)?;
        Ok(Vm {
            code: bytecode,
            pc: 0,
            stack: Vec::new(),
        })
    }

    pub fn next(&mut self) -> Option<Opcode> {
        let addr = self.pc;
        match self.code[addr] {
            0x00 => {
                self.pc += 1;
                Some(Opcode::STOP(addr))
            }
            0x01 => {
                self.pc += 1;
                Some(Opcode::ADD(addr))
            }
            0x02 => {
                self.pc += 1;
                Some(Opcode::MUL(addr))
            }
            0x60 => {
                let value = self.code[self.pc + 1];
                self.pc += 2;
                Some(Opcode::PUSH1(addr, value))
            }
            0x61 => {
                let value0 = self.code[self.pc + 1];
                let value1 = self.code[self.pc + 2];
                self.pc += 3;
                Some(Opcode::PUSH2(addr, value0, value1))
            }
            _ => {
                self.pc += 1;
                None
            }
        }
    }
    pub fn print_stack(&self) {
        self.stack.iter().enumerate().rev().for_each(|(i, x)| {
            let mut bytes = vec![0; 32];
            x.to_big_endian(&mut bytes);
            println!("|{}:\t{:?}", i, x);
        });
    }
    pub fn interpret(&mut self) {
        let maybe_op = self.next();
        match &maybe_op {
            Some(op) => op.describe(),
            None => println!("Unknown opcode"),
        }
        match &maybe_op {
            Some(x) => match x {
                Opcode::PUSH1(addr, value) => {
                    let value = U256::from(*value);
                    self.stack.push(value);
                }
                Opcode::ADD(addr) => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                _ => {}
            },
            None => {}
        }
    }
    pub fn at_end(&self) -> bool {
        self.pc >= self.code.len()
    }
}
