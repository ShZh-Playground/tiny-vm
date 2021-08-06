use crate::chunk::Chunk;
use crate::error::ExitStatus;
use crate::opcode::OpCode;

pub struct VM {
    chunk: Chunk,
    stack: Vec<f64>,
}

fn binary_operation(lvalue: f64, op: &OpCode, rvalue: f64) -> Option<f64> {
    match op {
        OpCode::OpAddition => Some(lvalue + rvalue),
        OpCode::OpSubtraction => Some(lvalue - rvalue),
        OpCode::OpMultiplication => Some(lvalue * rvalue),
        OpCode::OpDivision => Some(lvalue / rvalue),
        _ => None,
    }
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk: chunk,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> ExitStatus {
        let mut offset = 0;
        while offset < self.chunk.code.len() {
            let opcode = &self.chunk.code[offset];
            match opcode {
                OpCode::OpConstant => {
                    if let OpCode::Offset(index) = &self.chunk.code[offset + 1] {
                        let constant = self.chunk.constants_pool[*index];
                        self.stack.push(constant);
                    } else {
                        return ExitStatus::CompileTimeError;
                    }
                    offset += 2;
                    continue;
                }
                OpCode::OpReturn => {
                    println!("{}", self.stack.pop().unwrap());
                }
                OpCode::OpNegative => {
                    let push_value = self.stack.pop().unwrap();
                    self.stack.push(-push_value);
                }
                operation
                @
                (OpCode::OpAddition
                | OpCode::OpSubtraction
                | OpCode::OpMultiplication
                | OpCode::OpDivision) => {
                    let left_value = self.stack.pop().unwrap();
                    let right_value = self.stack.pop().unwrap();
                    let result = binary_operation(left_value, operation, right_value).unwrap();
                    self.stack.push(result);
                }
                _ => return ExitStatus::CompileTimeError,
            }
            offset += 1;
        }
        ExitStatus::Success
    }
}
