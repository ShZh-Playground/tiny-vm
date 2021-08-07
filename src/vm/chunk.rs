use super::opcode::OpCode;
use std::fmt;

pub struct Chunk {
    // Each element stand for one bytecode
    // Since constants are too large to store
    // We only store offset here, the number is in constants pool
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
    // TODO: the byte range limit the size of the constants pool
    pub constants_pool: Vec<f64>,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut offset = 0;
        while offset < self.code.len() {
            // The last line may be None if the current line is the first line
            // Return '|' if the last line and cur line are the same
            let line_char = if offset == 0 || self.lines[offset] != self.lines[offset - 1] {
                format!("{:>4}", self.lines[offset])
            } else {
                String::from("   |")
            };

            let opcode = &self.code[offset];
            match opcode {
                OpCode::OpConstant => {
                    if let OpCode::Offset(index) = &self.code[offset + 1] {
                        let constant = self.constants_pool[*index];
                        writeln!(f, "{:04} {} {:?} {}", offset, line_char, opcode, constant)?;
                        offset += 2;
                        continue;
                    } else {
                        // TODO: detect error in else where
                        return Err(fmt::Error);
                    }
                }
                _ => {
                    writeln!(f, "{:04} {} {:?}", offset, line_char, opcode)?;
                }
            }
            offset += 1;
        }
        Ok(())
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants_pool: Vec::new(),
        }
    }

    pub fn write_byte(&mut self, line: usize, operation: OpCode) {
        self.lines.push(line);
        self.code.push(operation);
    }

    pub fn write_constant(&mut self, line: usize, constant: f64) {
        self.lines.push(line);
        // Push constant and store index in bytecodes
        self.constants_pool.push(constant);
        let index = self.constants_pool.len() - 1;
        self.code.push(OpCode::Offset(index));
    }
}
