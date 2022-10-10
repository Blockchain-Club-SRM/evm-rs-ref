use bigint::U256;

#[derive(Debug)]
pub enum Opcode {
    STOP(usize),
    ADD(usize),
    SUB(usize),
    MUL(usize),
    PUSH1(usize, U256),
    PUSH2(usize, U256),
    PUSH32(usize, U256),
    MLOAD(usize),
    MSTORE(usize),
    MSTORE8(usize),
    SLT(usize),
    JUMPI(usize),
    JUMP(usize),
    PRINT(usize),
    EOF,
}

impl Opcode {
    pub fn describe(&self) {
        match self {
            Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::SUB(line) => println!("0x{:x}\tSUB\tSubtraction operation", line),
            Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            Opcode::PUSH1(line, x) => println!(
                "0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}",
                line, x
            ),
            Opcode::PUSH2(line, x) => println!(
                "0x{:x}\tPUSH2\tPlace 2-bytes item on the stack from 0x{:x}",
                line, x
            ),
            Opcode::SLT(line) => println!("0x{:x}\tSLT\tSigned less-than comparison", line),
            Opcode::JUMPI(line) => println!("0x{:x}\tJUMPI\tAlter the program counter", line),
            Opcode::JUMP(line) => println!("0x{:x}\tJUMP\tAlter the program counter", line),
            Opcode::PRINT(line) => println!("0x{:x}\tPRINT\tPrint the stack", line),
            Opcode::MLOAD(line) => println!("0x{:x}\tMLOAD\tLoad word from memory", line),
            Opcode::MSTORE(line) => println!("0x{:x}\tMSTORE\tSave word to memory", line),
            Opcode::MSTORE8(line) => println!("0x{:x}\tMSTORE8\tSave byte to memory", line),
            Opcode::PUSH32(line, x) => println!(
                "0x{:x}\tPUSH32\tPlace 32-bytes item on the stack from 0x{:x}",
                line, x
            ),
            Opcode::EOF => println!("EOF"),
            _ => println!("Unknown opcode"),
        }
    }
}
