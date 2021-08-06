mod chunk;
mod opcode;
mod vm;
mod error;

use opcode::OpCode;
use vm::VM;

fn main() {
    let mut chunks = chunk::Chunk::new();
    chunks.write_byte(123, OpCode::OpConstant);
    chunks.write_constant(123, 2_f64);
    chunks.write_byte(123, OpCode::OpConstant);
    chunks.write_constant(123, 3_f64);
    chunks.write_byte(123, OpCode::OpMultiplication);

    chunks.write_byte(123, OpCode::OpConstant);
    chunks.write_constant(123, 1_f64);
    chunks.write_byte(123, OpCode::OpAddition);
    chunks.write_byte(123, OpCode::OpNegative);
    chunks.write_byte(123, OpCode::OpReturn);

    let mut vm = VM::new(chunks);
    vm.run();
}
