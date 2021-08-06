use std::fmt;
use crate::opcode;

pub struct Chunk {
    pub code: Vec<opcode::OpCode>
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "== test chunk ==")?;
        for (offset, instruction) in self.code.iter().enumerate() {
            writeln!(f, "{:04} {:?}", offset, instruction)?;
        }
        Ok(())
    }
}