use super::memory::Memory;
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
    pub mem: Memory,
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
            mem: Memory::new(),
        })
    }
    pub fn get_new_size(&self, code: &Opcode) -> Option<usize> {
        match code {
            Opcode::MLOAD(_) | Opcode::MSTORE(_) => {
                Some(self.stack.last().unwrap().as_u64() as usize + 32)
            }
            Opcode::MSTORE8(_) => Some(self.stack.last().unwrap().as_u64() as usize + 1),
            _ => None,
        }
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
            0x03 => {
                self.pc += 1;
                Some(Opcode::SUB(addr))
            }
            0x60 => {
                let value = self.extract_u256(1);
                self.pc += 2;
                Some(Opcode::PUSH1(addr, value))
            }
            0x61 => {
                let value = self.extract_u256(2);
                self.pc += 3;
                Some(Opcode::PUSH2(addr, value))
            }
            0x73 => {
                let value = self.extract_u256(32);
                self.pc += 33;
                Some(Opcode::PUSH32(addr, value))
            }
            0x12 => {
                self.pc += 1;
                Some(Opcode::SLT(addr))
            }
            0x57 => {
                self.pc += 1;
                Some(Opcode::JUMPI(addr))
            }
            0x56 => {
                self.pc += 1;
                Some(Opcode::JUMP(addr))
            }
            0x51 => {
                self.pc += 1;
                Some(Opcode::MLOAD(addr))
            }
            0x52 => {
                self.pc += 1;
                Some(Opcode::MSTORE(addr))
            }
            0x53 => {
                self.pc += 1;
                Some(Opcode::MSTORE8(addr))
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
            println!("|{}:\t{:?}|", i, bytes);
        });
    }
    pub fn extract_u256(&mut self, to_extract: usize) -> U256 {
        let mut bytes = vec![0; 32];
        for i in 0..to_extract {
            let value = self.code[self.pc + i + 1];
            bytes[32 - to_extract + i] = value;
        }
        U256::from_big_endian(&bytes)
    }
    pub fn print_debug(&self) {
        println!("PC: {}", self.pc);
        println!("Stack:");
        self.print_stack();
    }
    pub fn interpret(&mut self) {
        let maybe_op = self.next();
        match &maybe_op {
            Some(op) => op.describe(),
            None => println!("Unknown opcode"),
        }
        // match self.get_new_size(&maybe_op) {
        //     Some(new_size) => self.mem.resize(new_size),
        //     None => (),
        // }
        match &maybe_op {
            Some(x) => match x {
                Opcode::PUSH1(_addr, value) => {
                    let value = U256::from(*value);
                    self.stack.push(value);
                }
                Opcode::ADD(_addr) => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                Opcode::SUB(_addr) => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                Opcode::SLT(_addr) => {
                    let lhs = self.stack.pop().unwrap();
                    let rhs = self.stack.pop().unwrap();
                    if lhs < rhs {
                        self.stack.push(U256::from(0x01));
                    } else {
                        self.stack.push(U256::from(0x00));
                    }
                }
                Opcode::JUMPI(_addr) => {
                    let then_addr = self.stack.pop().unwrap();
                    let cond = self.stack.pop().unwrap();
                    if !cond.is_zero() {
                        self.pc = then_addr.as_u64() as usize;
                    }
                }
                Opcode::PRINT(_addr) => {
                    let value = self.stack.pop().unwrap();
                    let mut bytes = vec![0; 32];
                    value.to_big_endian(&mut bytes);
                    println!("PRINT:\t{:?}|", value);
                }
                Opcode::JUMP(_addr) => {
                    let then_addr = self.stack.pop().unwrap();
                    self.pc = then_addr.as_u64() as usize;
                }
                Opcode::MLOAD(_addr) => {
                    let offset = self.stack.pop().unwrap();
                    let loaded_value = self.mem.get_word(offset.as_u64() as usize);
                    self.stack.push(loaded_value);
                }
                Opcode::MSTORE(_addr) => {
                    let offset = self.stack.pop().unwrap();
                    let value = self.stack.pop().unwrap();
                    self.mem.set_word(offset.as_u64() as usize, value);
                }
                Opcode::MSTORE8(_addr) => {
                    let offset = self.stack.pop().unwrap();
                    let value = self.stack.pop().unwrap().byte(31);
                    self.mem.set_byte(offset.as_u64() as usize, value);
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
