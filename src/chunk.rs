use std::fmt;
use crate::opcode::{ Operation, OpCode };

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants_pool: Vec<f64>,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "== test chunk ==")?;
        let mut offset = 0;
        for instruction in self.code.iter() {
            // Update bytecode offset
            match instruction {
                OpCode::OpReturn => {
                    writeln!(f, "{:04} {:?}", offset, instruction)?;
                    offset += 1;
                }
                OpCode::OpConstant(index) => {
                    writeln!(f, "{:04} {:?} {}", offset, instruction, self.constants_pool[*index])?;
                    offset += 2;
                }
            }
        }
        Ok(())
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants_pool: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, operation: Operation) {
        match operation {
            Operation::Return =>  self.code.push(OpCode::OpReturn),
            Operation::Constant(constant) => {
                self.constants_pool.push(constant);
                // Add instruction and index in constants pool
                let index = self.constants_pool.len() - 1;
                self.code.push(OpCode::OpConstant(index));
            }
        }
    }
}