mod chunk;
mod opcode;
mod vm;
mod error;

use opcode::OpCode;
use vm::VM;

fn main() {
    let mut chunks = chunk::Chunk::new();
    chunks.write_byte(123, OpCode::OpConstant);
    chunks.write_constant(123, 64_f64);
    chunks.write_byte(123, OpCode::OpReturn);

    let mut vm = VM::new(chunks);
    vm.run();
}
