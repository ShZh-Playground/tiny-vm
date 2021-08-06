use crate::chunk::Chunk;
use crate::opcode::OpCode;

pub struct VM {
    chunk: Chunk,
    stack: Vec<f64>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk: chunk,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        for (_, instruction) in self.chunk.code.iter() {
            match instruction {
                OpCode::OpConstant(offset) => {
                    let constant = self.chunk.constants_pool[*offset];
                    self.stack.push(constant);
                },
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                }
            }
        }
    }
}