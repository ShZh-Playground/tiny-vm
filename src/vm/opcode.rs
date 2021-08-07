#[derive(Debug)]
pub enum OpCode {
    Offset(usize),
    OpReturn,
    OpConstant,
    OpNegative,
    OpAddition,
    OpSubtraction,
    OpMultiplication,
    OpDivision,
}
