mod chunk;
mod opcode;

fn main() {
    let chunks = chunk::Chunk{ code: vec!(opcode::OpCode::OpReturn) };
    println!("{}", chunks);
}
