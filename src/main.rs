mod evm;
// use evm::opcode::Opcode;
use evm::vm::Vm;
use std::error::Error;
use std::env;

fn debug(vm: &mut Vm) {
    loop {
        match vm.next() {
            None => break,
            Some(opcode) => opcode.describe(),
        }
    }
}
fn interpret(vm: &mut Vm) {
    while !vm.at_end() {
        vm.interpret();
    }
    vm.print_stack();
}
fn run() -> Result<(), Box<dyn Error>> {
    let args:Vec<String> = env::args().collect();
    let function = args[1].clone();
    let filename = args[2].clone();
    print!("Reading bytecode from {} ... ", filename);
    let mut vm = Vm::new_from_file(&filename)?;
    println!("Correctly loaded VM");
    match &*function {
        "debug" => debug(&mut vm),
        "interpret" => interpret(&mut vm),
        _ => panic!("Expect either 'debug' or 'run' for first parameter")
    }
    Ok(())
}

fn main() {
    run().unwrap();
}