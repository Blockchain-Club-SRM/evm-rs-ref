use super::vm::Vm;

fn create_vm(binary: Vec<u8>) -> Vm {
    Vm {
        code: binary,
        pc: 0,
        stack: Vec::new(),
    }
}

#[test]
fn addition() {
    let binary = vec![0x60, 0x0f, 0x60, 0x01, 0x01, 0x00];
    let mut vm = create_vm(binary);
    vm.interpret();
    vm.interpret();
    vm.interpret();
    vm.interpret();
    assert_eq!(1,vm.stack.len());
    assert_eq!(16, vm.stack[0].as_u32());
}
