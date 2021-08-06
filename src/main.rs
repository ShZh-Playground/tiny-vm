mod chunk;
mod opcode;

use opcode::Operation;

fn main() {
    let mut chunks = chunk::Chunk::new();
    chunks.add_instruction(Operation::Constant(64_f64));
    chunks.add_instruction(Operation::Return);

    println!("{}", chunks);
}
