mod chunk;
mod opcode;
mod vm;

use opcode::Operation;
use vm::VM;

fn main() {
    let mut chunks = chunk::Chunk::new();
    chunks.add_instruction(123, Operation::Constant(64_f64));
    chunks.add_instruction(123, Operation::Return);

    let mut vm = VM::new(chunks);
    vm.run();
}
