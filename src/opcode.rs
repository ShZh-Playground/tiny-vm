pub enum Operation {
    Return,
    Constant(f64),
}

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
}
