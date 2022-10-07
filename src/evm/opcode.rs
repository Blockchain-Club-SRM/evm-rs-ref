#[derive(Debug)]
pub enum Opcode {
    STOP(usize),
    ADD(usize),
    MUL(usize),
    PUSH1(usize, u8),
    PUSH2(usize, u8, u8),
    // PUSH32(
    //     usize,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    //     u8,
    // ),
    EOF,
}

impl Opcode {
    pub fn describe(&self) {
        match self {
            Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            Opcode::PUSH1(line, x) => println!(
                "0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}",
                line, x
            ),
            Opcode::PUSH2(line, x0, x1) => println!(
                "0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}",
                line, x0, x1
            ),
            _ => println!("Unknown opcode"),
        }
    }
}
