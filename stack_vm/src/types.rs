pub type OperandType = i64;
pub type VariableNameType = char;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    OpLoadVal(OperandType),
    OpReadVar(VariableNameType),
    OpWriteVar(VariableNameType),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpReturnValue,
}

pub struct Stack(pub Vec<OperandType>);
impl Stack {
    pub fn push(&mut self, v: OperandType) {
        self.0.push(v);
    }

    pub fn pop(&mut self) -> OperandType{
        self.0.pop().expect("popped an empty stack")
    }
}

pub type Program = Vec<Opcode>;