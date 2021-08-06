use crate::opcode::{OpCode, Operation};
use std::fmt;

pub struct Chunk {
    pub code: Vec<(usize, OpCode)>,
    pub constants_pool: Vec<f64>,
}

// The last line may be None if the current line is the first line
// Return '|' if the last line and cur line are the same
fn get_line_char(last_line: Option<usize>, cur_line: usize) -> String {
    if let Some(last_line) = last_line {
        if cur_line == last_line {
            String::from("   |")
        } else {
            format!("{:>4}", cur_line)
        }
    } else {
        format!("{:>4}", cur_line)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "== test chunk ==")?;
        let mut offset = 0;
        let mut last_line: Option<usize> = None;
        for (line, instruction) in self.code.iter() {
            let line_char = get_line_char(last_line, *line);

            // Print instruction and
            // update bytecode offset
            match instruction {
                OpCode::OpReturn => {
                    writeln!(f, "{:04} {} {:?}", offset, line_char, instruction)?;
                    offset += 1;
                }
                OpCode::OpConstant(index) => {
                    writeln!(
                        f,
                        "{:04} {} {:?} {}",
                        offset, line_char, instruction, self.constants_pool[*index]
                    )?;
                    offset += 2;
                }
            }

            last_line = Some(*line);
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

    pub fn add_instruction(&mut self, line: usize, operation: Operation) {
        match operation {
            Operation::Return => self.code.push((line, OpCode::OpReturn)),
            Operation::Constant(constant) => {
                self.constants_pool.push(constant);
                // Add instruction and index in constants pool
                let index = self.constants_pool.len() - 1;
                self.code.push((line, OpCode::OpConstant(index)));
            }
        }
    }
}
